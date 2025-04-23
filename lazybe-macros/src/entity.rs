use convert_case::{Case, Casing};
use darling::{FromDeriveInput, FromField};
use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::{Data, DataStruct, DeriveInput, Fields, FieldsNamed, Ident, Type, Visibility};

use crate::common::{self, CollectionApi, ValidationHook};

#[derive(Clone, FromDeriveInput)]
#[darling(attributes(lazybe))]
struct EntityAttr {
    table: String,
    #[darling(default)]
    endpoint: Option<String>,
    #[darling(default)]
    collection_api: CollectionApi,
    #[darling(default)]
    validation: ValidationHook,
    #[darling(default)]
    derive_to_schema: bool,
}

#[derive(Clone, FromField)]
#[darling(attributes(lazybe))]
struct EntityFieldAttr {
    #[darling(default)]
    primary_key: bool,
    #[darling(default)]
    generate_with: Option<String>,
    #[darling(default)]
    created_at: bool,
    #[darling(default)]
    updated_at: bool,
    #[darling(default)]
    json: bool,
}

#[derive(Clone)]
struct EntityField {
    vis: Visibility,
    ident: Ident,
    ident_pascal: Ident,
    ty: Type,
    attr: EntityFieldAttr,
}

struct EntityMeta {
    entity_ident: Ident,
    entity_vis: Visibility,
    attr: EntityAttr,
    create_entity: Ident,
    update_entity: Ident,
    replace_entity: Ident,
    filter_entity: Ident,
    sort_entity: Ident,
    sqlx_row_ident: Ident,
    sea_query_ident: Ident,
    primary_key: EntityField,
    created_at: Option<EntityField>,
    updated_at: Option<EntityField>,
    all_fields: Vec<EntityField>,
    user_defined_fields: Vec<EntityField>,
}

impl EntityMeta {
    fn try_parse(input: &DeriveInput, fields: &FieldsNamed) -> syn::Result<Self> {
        let parsed_fields = fields
            .named
            .iter()
            .map(|f| {
                f.ident
                    .as_ref()
                    .ok_or(syn::Error::new_spanned(&input.ident, "Only name struct is supported"))
                    .map(|ident| (f, ident))
            })
            .collect::<Result<Vec<_>, _>>()?
            .into_iter()
            .map(|(field, ident)| {
                EntityFieldAttr::from_field(field).map(|attr| EntityField {
                    vis: field.vis.clone(),
                    ident: ident.clone(),
                    ident_pascal: format_ident!("{}", ident.to_string().to_case(Case::Pascal)),
                    ty: field.ty.clone(),
                    attr,
                })
            })
            .collect::<Result<Vec<_>, _>>()?;
        Ok(EntityMeta {
            entity_ident: input.ident.clone(),
            entity_vis: input.vis.clone(),
            attr: EntityAttr::from_derive_input(input)?,
            create_entity: format_ident!("Create{}", input.ident),
            update_entity: format_ident!("Update{}", input.ident),
            replace_entity: format_ident!("Replace{}", input.ident),
            filter_entity: format_ident!("{}Filter", input.ident),
            sort_entity: format_ident!("{}Sort", input.ident),
            sqlx_row_ident: format_ident!("{}SqlxRow", input.ident),
            sea_query_ident: format_ident!("{}SeaQueryIdent", input.ident),
            primary_key: Self::detect_primary_key(&input.ident, &parsed_fields)?,
            created_at: Self::detect_created_at(&input.ident, &parsed_fields)?,
            updated_at: Self::detect_updated_at(&input.ident, &parsed_fields)?,
            all_fields: parsed_fields.to_vec(),
            user_defined_fields: parsed_fields
                .iter()
                .filter(|field| !field.attr.primary_key)
                .filter(|field| !field.attr.created_at)
                .filter(|field| !field.attr.updated_at)
                .cloned()
                .collect(),
        })
    }

    fn detect_primary_key(entity: &Ident, all_fields: &[EntityField]) -> syn::Result<EntityField> {
        let mut maybe_pk = None;
        for field in all_fields {
            if field.attr.primary_key {
                match maybe_pk {
                    Some(_) => Err(syn::Error::new_spanned(entity, "Exactly 1 field must be primary key"))?,
                    None => maybe_pk = Some(field.clone()),
                }
            }
        }
        maybe_pk.ok_or_else(|| syn::Error::new_spanned(entity, "Exactly 1 field must be primary key"))
    }

    fn detect_created_at(entity: &Ident, all_fields: &[EntityField]) -> syn::Result<Option<EntityField>> {
        let mut maybe_field = None;
        for field in all_fields {
            if field.attr.created_at {
                match maybe_field {
                    Some(_) => Err(syn::Error::new_spanned(
                        entity,
                        "No more than 1 field can be created_at",
                    ))?,
                    None => maybe_field = Some(field.clone()),
                }
            }
        }
        Ok(maybe_field)
    }

    fn detect_updated_at(entity: &Ident, all_fields: &[EntityField]) -> syn::Result<Option<EntityField>> {
        let mut maybe_field = None;
        for field in all_fields {
            if field.attr.updated_at {
                match maybe_field {
                    Some(_) => Err(syn::Error::new_spanned(
                        entity,
                        "No more than 1 field can be updated_at",
                    ))?,
                    None => maybe_field = Some(field.clone()),
                }
            }
        }
        Ok(maybe_field)
    }
}

pub fn expand(input: DeriveInput) -> TokenStream {
    match &input.data {
        Data::Struct(data_struct) => expand_struct(&input, data_struct).unwrap_or_else(|e| e.into_compile_error()),
        Data::Enum(_) => {
            syn::Error::new_spanned(&input.ident, "Enum is not supported by the provided macro").into_compile_error()
        }
        Data::Union(_) => {
            syn::Error::new_spanned(&input.ident, "Union is not supported by the provided macro").into_compile_error()
        }
    }
}

fn expand_struct(input: &DeriveInput, data_struct: &DataStruct) -> syn::Result<TokenStream> {
    match &data_struct.fields {
        Fields::Named(fields_named) => {
            let entity_meta = EntityMeta::try_parse(input, fields_named)?;
            let mut ts = TokenStream::new();
            ts.extend(entity_types_impl(&entity_meta));
            ts.extend(entity_row_impl(&entity_meta));
            ts.extend(entity_entity_trait_impl(&entity_meta));
            ts.extend(entity_query_trait_impl(&entity_meta));
            ts.extend(entity_route_trait_impl(&entity_meta));
            ts.extend(entity_collection_api_trait_impl(&entity_meta));
            ts.extend(entity_validation_hook_trait_impl(&entity_meta));
            Ok(ts)
        }
        Fields::Unnamed(_) => Err(syn::Error::new_spanned(
            &input.ident,
            "Tuple struct is not supported by the provided macro",
        ))?,
        Fields::Unit => Err(syn::Error::new_spanned(
            &input.ident,
            "Unit struct is not supported by the provided macro",
        ))?,
    }
}

fn entity_types_impl(entity_meta: &EntityMeta) -> TokenStream {
    let entity = &entity_meta.entity_ident;
    let entity_vis = &entity_meta.entity_vis;
    let entity_sea_query_ident = &entity_meta.sea_query_ident;
    let create_entity = &entity_meta.create_entity;
    let patch_entity = &entity_meta.update_entity;
    let put_entity = &entity_meta.replace_entity;
    let filter_entity = &entity_meta.filter_entity;
    let sort_entity = &entity_meta.sort_entity;
    let user_defined_field_defs = entity_meta.user_defined_fields.iter().map(|f| {
        let field_vis = &f.vis;
        let field_ident = &f.ident;
        let field_ty = &f.ty;
        quote! { #field_vis #field_ident: #field_ty }
    });
    let patch_entity_field_defs = entity_meta.user_defined_fields.iter().map(|f| {
        let field_vis = &f.vis;
        let field_ident = &f.ident;
        let field_ty = &f.ty;
        quote! { #field_vis #field_ident: Option<#field_ty> }
    });
    let put_entity_field_defs = entity_meta.user_defined_fields.iter().map(|f| {
        let field_vis = &f.vis;
        let field_ident = &f.ident;
        let field_ty = &f.ty;
        quote! { #field_vis #field_ident: #field_ty }
    });
    let put_entity_field_intos = entity_meta.user_defined_fields.iter().map(|f| {
        let field_ident = &f.ident;
        quote! { #field_ident: Some(value.#field_ident) }
    });
    let filter_method_defs = entity_meta.all_fields.iter().map(|f| {
        let field_ident = &f.ident;
        let field_ident_pascal = &f.ident_pascal;
        let field_ty = &f.ty;
        quote! {
            pub fn #field_ident() -> lazybe::filter::FilterCol<#entity, #field_ty> {
                lazybe::filter::FilterCol::new(#entity_sea_query_ident::#field_ident_pascal)
            }
        }
    });
    let sort_method_defs = entity_meta.all_fields.iter().map(|f| {
        let field_ident = &f.ident;
        let field_ident_pascal = &f.ident_pascal;
        quote! {
            pub fn #field_ident() -> lazybe::sort::SortCol<#entity> {
                lazybe::sort::SortCol::new(#entity_sea_query_ident::#field_ident_pascal)
            }
        }
    });
    let derive_to_schema = Some(quote! { #[derive(utoipa::ToSchema)] }).filter(|_| entity_meta.attr.derive_to_schema);
    quote! {
        #[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
        #derive_to_schema
        #entity_vis struct #create_entity {
            #(#user_defined_field_defs),*
        }

        #[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
        #derive_to_schema
        #entity_vis struct #patch_entity {
            #(#patch_entity_field_defs),*
        }

        #[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
        #derive_to_schema
        #entity_vis struct #put_entity {
            #(#put_entity_field_defs),*
        }

        impl From<#put_entity> for #patch_entity {
            fn from(value: #put_entity) -> Self {
                Self {
                    #(#put_entity_field_intos),*
                }
            }
        }

        #[derive(Debug, Clone)]
        #entity_vis struct #filter_entity;

        impl #filter_entity {
            #(#filter_method_defs)*
        }

        #[derive(Debug, Clone)]
        #entity_vis struct #sort_entity;

        impl #sort_entity {
            #(#sort_method_defs)*
        }
    }
}

fn entity_row_impl(entity_meta: &EntityMeta) -> TokenStream {
    let entity = &entity_meta.entity_ident;
    let entity_vis = &entity_meta.entity_vis;
    let sqlx_row_ident = &entity_meta.sqlx_row_ident;
    let sea_query_ident = &entity_meta.sea_query_ident;
    let table_name = entity_meta.attr.table.to_string();
    let all_field_idents = entity_meta.all_fields.iter().map(|f| {
        let ident_str = f.ident.to_string();
        let ident_pascal = &f.ident_pascal;
        quote! {
            #[iden = #ident_str]
            #ident_pascal
        }
    });
    let all_field_defs = entity_meta.all_fields.iter().map(|f| {
        let ident = &f.ident;
        let ty = &f.ty;
        if f.attr.json {
            quote! { #ident: sqlx::types::Json<#ty> }
        } else {
            quote! { #ident: #ty }
        }
    });
    let all_field_intos = entity_meta.all_fields.iter().map(|f| {
        let ident = &f.ident;
        if f.attr.json {
            quote! { #ident: value.#ident.0 }
        } else {
            quote! { #ident: value.#ident }
        }
    });
    quote! {
        #[derive(sqlx::FromRow)]
        #entity_vis struct #sqlx_row_ident {
            #(#all_field_defs),*
        }

        impl From<#sqlx_row_ident> for #entity {
            fn from(value: #sqlx_row_ident) -> Self {
                Self {
                    #(#all_field_intos),*
                }
            }
        }

        #[derive(sea_query::Iden)]
        #[iden = #table_name]
        enum #sea_query_ident {
            Table,
            #(#all_field_idents),*
        }
    }
}

fn entity_route_trait_impl(entity_meta: &EntityMeta) -> TokenStream {
    let entity = &entity_meta.entity_ident;
    let Some(base_url) = entity_meta.attr.endpoint.as_ref() else {
        return TokenStream::new();
    };
    common::entity_route_trait_impl(entity, base_url)
}

fn entity_collection_api_trait_impl(entity_meta: &EntityMeta) -> TokenStream {
    if entity_meta.attr.endpoint.is_none() {
        return TokenStream::new();
    }
    common::entity_collection_api_trait_impl(
        &entity_meta.entity_ident,
        &entity_meta.attr.collection_api,
        Some((&entity_meta.primary_key.ident, &entity_meta.sort_entity)),
    )
}

fn entity_validation_hook_trait_impl(entity_meta: &EntityMeta) -> TokenStream {
    if entity_meta.attr.endpoint.is_none() {
        return TokenStream::new();
    }
    common::entity_validation_hook_trait_impl(&entity_meta.entity_ident, &entity_meta.attr.validation)
}

fn entity_entity_trait_impl(entity_meta: &EntityMeta) -> TokenStream {
    let entity = &entity_meta.entity_ident;
    let entity_str = entity.to_string();
    let pk_ty = &entity_meta.primary_key.ty;
    let create_entity = &entity_meta.create_entity;
    let update_entity = &entity_meta.update_entity;
    let replace_entity = &entity_meta.replace_entity;
    let row_entity = &entity_meta.sqlx_row_ident;
    quote! {
        impl lazybe::Entity for #entity {
            type Pk = #pk_ty;
            type Create = #create_entity;
            type Update = #update_entity;
            type Replace = #replace_entity;

            fn entity_name() -> &'static str {
                #entity_str
            }
        }

        impl lazybe::TableEntity for #entity {
            type Row = #row_entity;
        }
    }
}

fn entity_query_trait_impl(entity_meta: &EntityMeta) -> TokenStream {
    let mut ts = TokenStream::new();
    ts.extend(get_query_trait_impl(entity_meta));
    ts.extend(list_query_trait_impl(entity_meta));
    ts.extend(create_query_trait_impl(entity_meta));
    ts.extend(update_query_trait_impl(entity_meta));
    ts.extend(delete_query_trait_impl(entity_meta));
    ts
}

fn create_query_trait_impl(entity_meta: &EntityMeta) -> TokenStream {
    let entity = &entity_meta.entity_ident;
    let sea_query_ident = &entity_meta.sea_query_ident;
    let user_defined_fields_value = entity_meta.user_defined_fields.iter().map(|f| {
        let ident = &f.ident;
        if f.attr.json {
            quote! { serde_json::to_value(input.#ident).unwrap().into() }
        } else {
            quote! { input.#ident.into() }
        }
    });
    let user_defined_fields_ident_pascal = entity_meta.user_defined_fields.iter().map(|i| i.ident_pascal.clone());
    let now_value = Some(quote! { let now = sqlx::types::chrono::Utc::now(); })
        .filter(|_| entity_meta.created_at.is_some() || entity_meta.updated_at.is_some());
    let created_at_ident_pascal = entity_meta
        .created_at
        .as_ref()
        .map(|f| {
            let ident = &f.ident_pascal;
            quote! { #sea_query_ident::#ident }
        })
        .into_iter();
    let updated_at_ident_pascal = entity_meta
        .updated_at
        .as_ref()
        .map(|f| {
            let ident = &f.ident_pascal;
            quote! { #sea_query_ident::#ident }
        })
        .into_iter();
    let created_at_value = entity_meta
        .created_at
        .as_ref()
        .map(|_| quote! { now.into() })
        .into_iter();
    let updated_at_value = entity_meta
        .updated_at
        .as_ref()
        .map(|_| quote! { now.into() })
        .into_iter();
    let pk_ident_pascal = entity_meta
        .primary_key
        .attr
        .generate_with
        .as_ref()
        .map(|_| {
            let pk_ident = &entity_meta.primary_key.ident_pascal;
            quote! { #sea_query_ident::#pk_ident }
        })
        .into_iter();
    let pk_value = entity_meta
        .primary_key
        .attr
        .generate_with
        .as_ref()
        .map(|f| {
            let f_ident = format_ident!("{}", f);
            quote! {
                (#f_ident(&input).into())
            }
        })
        .into_iter();

    quote! {
        impl lazybe::query::CreateQuery for #entity {
            fn create_query(input: Self::Create) -> sea_query::InsertStatement {
                #now_value
                sea_query::Query::insert()
                    .into_table(#sea_query_ident::Table)
                    .columns([
                        #(#pk_ident_pascal,)*
                        #(#sea_query_ident::#user_defined_fields_ident_pascal,)*
                        #(#created_at_ident_pascal,)*
                        #(#updated_at_ident_pascal,)*
                    ])
                   .values_panic([
                       #(#pk_value,)*
                        #(#user_defined_fields_value,)*
                        #(#created_at_value,)*
                        #(#updated_at_value,)*
                    ])
                    .returning_all()
                    .to_owned()
            }
        }
    }
}

fn update_query_trait_impl(entity_meta: &EntityMeta) -> TokenStream {
    let entity = &entity_meta.entity_ident;
    let sea_query_ident = &entity_meta.sea_query_ident;
    let pk_ident_pascal = &entity_meta.primary_key.ident_pascal;
    let now_value = Some(quote! { let now = sqlx::types::chrono::Utc::now(); })
        .filter(|_| entity_meta.created_at.is_some() || entity_meta.updated_at.is_some());
    let update_user_defined_fields = entity_meta.user_defined_fields.iter().map(|f| {
        let ident = &f.ident;
        let ident_pascal = &f.ident_pascal;
        if f.attr.json {
            quote! {
                if let Some(new_value) = input.#ident {
                    let json = serde_json::to_value(new_value).unwrap();
                    values.push((#sea_query_ident::#ident_pascal, json.into()));
                }
            }
        } else {
            quote! {
                if let Some(new_value) = input.#ident {
                    values.push((#sea_query_ident::#ident_pascal, new_value.into()));
                }
            }
        }
    });
    let update_updated_at = entity_meta
        .updated_at
        .as_ref()
        .map(|f| {
            let ident = &f.ident_pascal;
            quote! { values.push((#sea_query_ident::#ident, now.into())); }
        })
        .into_iter();
    quote! {
        impl lazybe::query::UpdateQuery for #entity {
            fn update_query(id: Self::Pk, input: Self::Update) -> sea_query::UpdateStatement {
                #now_value

                let mut values = Vec::new();
                #(#update_user_defined_fields)*
                #(#update_updated_at)*

                sea_query::Query::update()
                    .table(#sea_query_ident::Table)
                    .values(values)
                    .cond_where(
                        sea_query::Cond::all()
                            .add(sea_query::Expr::col(#sea_query_ident::#pk_ident_pascal).eq(id))
                    )
                    .returning_all()
                    .to_owned()
            }
        }
    }
}

fn get_query_trait_impl(entity_meta: &EntityMeta) -> TokenStream {
    let entity = &entity_meta.entity_ident;
    let sea_query_ident = &entity_meta.sea_query_ident;
    let pk_ident_pascal = &entity_meta.primary_key.ident_pascal;
    let all_field_idents_pascal = entity_meta.all_fields.iter().map(|f| f.ident_pascal.clone());
    quote! {
        impl lazybe::query::GetQuery for #entity {
            fn get_query(id: Self::Pk) -> sea_query::SelectStatement {
                sea_query::Query::select()
                    .columns([
                        #(#sea_query_ident::#all_field_idents_pascal),*
                    ])
                    .from(#sea_query_ident::Table)
                    .cond_where(
                        sea_query::Cond::all()
                            .add(sea_query::Expr::col(#sea_query_ident::#pk_ident_pascal).eq(id))
                    )
                    .to_owned()
            }
        }
    }
}

fn list_query_trait_impl(entity_meta: &EntityMeta) -> TokenStream {
    let entity = &entity_meta.entity_ident;
    let sea_query_ident = &entity_meta.sea_query_ident;
    let all_field_idents_pascal = entity_meta.all_fields.iter().map(|f| f.ident_pascal.clone());
    quote! {
        impl lazybe::query::ListQuery for #entity {
            fn list_query(filter: lazybe::filter::Filter<Self>) -> sea_query::SelectStatement {
                sea_query::Query::select()
                    .columns([
                        #(#sea_query_ident::#all_field_idents_pascal),*
                    ])
                    .from(#sea_query_ident::Table)
                    .cond_where(sea_query::Cond::all().add(filter))
                    .to_owned()
            }
        }
    }
}

fn delete_query_trait_impl(entity_meta: &EntityMeta) -> TokenStream {
    let entity = &entity_meta.entity_ident;
    let sea_query_ident = &entity_meta.sea_query_ident;
    let pk_ident_pascal = &entity_meta.primary_key.ident_pascal;
    quote! {
        impl lazybe::query::DeleteQuery for #entity {
            fn delete_query(id: Self::Pk) -> sea_query::DeleteStatement {
                sea_query::Query::delete()
                    .from_table(#sea_query_ident::Table)
                    .cond_where(
                        sea_query::Cond::all()
                            .add(sea_query::Expr::col(#sea_query_ident::#pk_ident_pascal).eq(id))
                    )
                    .to_owned()
            }
        }
    }
}
