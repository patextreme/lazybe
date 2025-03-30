use convert_case::{Case, Casing};
use darling::{FromDeriveInput, FromField};
use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::{Data, DataStruct, DeriveInput, Field, Fields, FieldsNamed, Ident, Visibility};

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

#[derive(Clone, FromDeriveInput)]
#[darling(attributes(lazybe))]
struct EntityAttr {
    table: String,
}

#[derive(Clone, FromField)]
#[darling(attributes(lazybe))]
struct EntityFieldAttr {
    #[darling(default)]
    primary_key: bool,
    #[darling(default)]
    created_at: bool,
    #[darling(default)]
    updated_at: bool,
}

struct EntityMeta {
    entity_ident: Ident,
    entity_vis: Visibility,
    entity_attr: EntityAttr,
    create_entity: Ident,
    sqlx_row_ident: Ident,
    sea_query_ident: Ident,
    primary_key: Field,
    created_at: Option<Field>,
    updated_at: Option<Field>,
    all_entity_fields: Vec<(Field, EntityFieldAttr)>,
    create_entity_fields: Vec<(Field, EntityFieldAttr)>,
}

impl EntityMeta {
    fn try_parse(input: &DeriveInput, fields: &FieldsNamed) -> syn::Result<Self> {
        let parsed_fields: Vec<(Field, EntityFieldAttr)> = fields
            .named
            .iter()
            .map(|f| EntityFieldAttr::from_field(f).map(|attr| (f.clone(), attr)))
            .collect::<Result<_, _>>()?;
        Ok(EntityMeta {
            entity_ident: input.ident.clone(),
            entity_vis: input.vis.clone(),
            entity_attr: EntityAttr::from_derive_input(input)?,
            create_entity: format_ident!("Create{}", input.ident),
            sqlx_row_ident: format_ident!("{}SqlxRow", input.ident),
            sea_query_ident: format_ident!("{}SeaQueryIdent", input.ident),
            primary_key: Self::detect_primary_key(&input.ident, &parsed_fields)?,
            created_at: Self::detect_created_at(&input.ident, &parsed_fields)?,
            updated_at: Self::detect_updated_at(&input.ident, &parsed_fields)?,
            all_entity_fields: parsed_fields.to_vec(),
            create_entity_fields: parsed_fields
                .iter()
                .filter(|(_, attr)| !attr.primary_key)
                .filter(|(_, attr)| !attr.created_at)
                .filter(|(_, attr)| !attr.updated_at)
                .cloned()
                .collect(),
        })
    }

    fn detect_primary_key(entity: &Ident, all_fields: &[(Field, EntityFieldAttr)]) -> syn::Result<Field> {
        let mut maybe_pk = None;
        for (field, attr) in all_fields {
            if attr.primary_key {
                match maybe_pk {
                    Some(_) => Err(syn::Error::new_spanned(entity, "Exactly 1 field must be primary key"))?,
                    None => maybe_pk = Some(field.clone()),
                }
            }
        }
        maybe_pk.ok_or_else(|| syn::Error::new_spanned(entity, "Exactly 1 field must be primary key"))
    }

    fn detect_created_at(entity: &Ident, all_fields: &[(Field, EntityFieldAttr)]) -> syn::Result<Option<Field>> {
        let mut maybe_field = None;
        for (field, attr) in all_fields {
            if attr.created_at {
                match maybe_field {
                    Some(_) => Err(syn::Error::new_spanned(
                        entity,
                        "No more then 1 field can be created_at",
                    ))?,
                    None => maybe_field = Some(field.clone()),
                }
            }
        }
        Ok(maybe_field)
    }

    fn detect_updated_at(entity: &Ident, all_fields: &[(Field, EntityFieldAttr)]) -> syn::Result<Option<Field>> {
        let mut maybe_field = None;
        for (field, attr) in all_fields {
            if attr.updated_at {
                match maybe_field {
                    Some(_) => Err(syn::Error::new_spanned(
                        entity,
                        "No more then 1 field can be updated_at",
                    ))?,
                    None => maybe_field = Some(field.clone()),
                }
            }
        }
        Ok(maybe_field)
    }
}

fn expand_struct(input: &DeriveInput, data_struct: &DataStruct) -> syn::Result<TokenStream> {
    match &data_struct.fields {
        Fields::Named(fields_named) => {
            let entity_meta = EntityMeta::try_parse(input, fields_named)?;
            let entity_types_impl = entity_types_impl(&entity_meta);
            let entity_row_impl = entity_row_impl(&entity_meta);
            let entity_query_trait_impl = entity_query_trait_impl(&entity_meta);
            Ok(quote! {
                #entity_types_impl
                #entity_query_trait_impl
                #entity_row_impl
            })
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
    let entity_vis = &entity_meta.entity_vis;
    let create_entity = &entity_meta.create_entity;
    let create_entity_fields = entity_meta.create_entity_fields.iter().map(|f| {
        let field_vis = &f.0.vis;
        let field_ident = f.0.ident.as_ref().unwrap();
        let field_ty = &f.0.ty;
        quote! { #field_vis #field_ident: #field_ty }
    });
    quote! {
        #entity_vis struct #create_entity {
            #(#create_entity_fields),*
        }
    }
}

fn entity_row_impl(entity_meta: &EntityMeta) -> TokenStream {
    let entity = &entity_meta.entity_ident;
    let entity_vis = &entity_meta.entity_vis;
    let sqlx_row_ident = &entity_meta.sqlx_row_ident;
    let sea_query_ident = &entity_meta.sea_query_ident;
    let table_name = entity_meta.entity_attr.table.to_string();
    let all_field_idents = entity_meta.all_entity_fields.iter().map(|(f, _)| {
        let ident_str = f.ident.as_ref().unwrap().to_string();
        let ident_pascal = format_ident!("{}", ident_str.to_case(Case::Pascal));
        quote! {
            #[iden = #ident_str]
            #ident_pascal
        }
    });
    let all_field_defs = entity_meta.all_entity_fields.iter().map(|(f, _)| {
        let ident = f.ident.as_ref().unwrap();
        let ty = &f.ty;
        quote! { #ident: #ty }
    });
    let all_field_intos = entity_meta.all_entity_fields.iter().map(|(f, _)| {
        let ident = f.ident.as_ref().unwrap();
        quote! { #ident: value.#ident }
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

fn entity_query_trait_impl(entity_meta: &EntityMeta) -> TokenStream {
    let entity = &entity_meta.entity_ident;
    let sqlx_row_ident = &entity_meta.sqlx_row_ident;
    let sea_query_ident = &entity_meta.sea_query_ident;
    let create_entity = &entity_meta.create_entity;
    let all_field_idents_pascal = entity_meta
        .all_entity_fields
        .iter()
        .flat_map(|f| f.0.ident.as_ref())
        .map(|i| format_ident!("{}", i.to_string().to_case(Case::Pascal)))
        .collect::<Vec<_>>();
    let create_field_idents_pascal = entity_meta
        .create_entity_fields
        .iter()
        .flat_map(|f| f.0.ident.as_ref())
        .map(|i| format_ident!("{}", i.to_string().to_case(Case::Pascal)))
        .collect::<Vec<_>>();
    let create_field_idents = entity_meta
        .create_entity_fields
        .iter()
        .flat_map(|f| f.0.ident.as_ref())
        .collect::<Vec<_>>();
    let pk_ty = &entity_meta.primary_key.ty;
    let pk_ident_pascal = format_ident!(
        "{}",
        &entity_meta
            .primary_key
            .ident
            .as_ref()
            .unwrap()
            .to_string()
            .to_case(Case::Pascal)
    );
    let created_at_ident_pascal = entity_meta
        .created_at
        .as_ref()
        .and_then(|f| f.ident.as_ref())
        .map(|i| format_ident!("{}", i.to_string().to_case(Case::Pascal)))
        .map(|ident| quote! { #sea_query_ident::#ident })
        .into_iter();
    let updated_at_ident_pascal = entity_meta
        .updated_at
        .as_ref()
        .and_then(|f| f.ident.as_ref())
        .map(|i| format_ident!("{}", i.to_string().to_case(Case::Pascal)))
        .map(|ident| quote! { #sea_query_ident::#ident })
        .into_iter();
    let now_value = Some(quote! { let now = sqlx::types::chrono::Utc::now(); })
        .filter(|_| entity_meta.created_at.is_some() || entity_meta.updated_at.is_some());
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

    quote! {
        impl lazybe::CreateQuery for #entity {
            type Create = #create_entity;
            type Row = #sqlx_row_ident;
            fn create_query(input: Self::Create) -> sea_query::InsertStatement {
                #now_value
                sea_query::Query::insert()
                    .into_table(#sea_query_ident::Table)
                    .columns([
                        #(#sea_query_ident::#create_field_idents_pascal,)*
                        #(#created_at_ident_pascal,)*
                        #(#updated_at_ident_pascal,)*
                    ])
                   .values_panic([
                        #(input.#create_field_idents.into(),)*
                        #(#created_at_value,)*
                        #(#updated_at_value,)*
                    ])
                    .returning_all()
                    .to_owned()
            }
        }

        impl lazybe::GetQuery for #entity {
            type Pk = #pk_ty;
            type Row = #sqlx_row_ident;
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

        impl lazybe::DeleteQuery for #entity {
            type Pk = #pk_ty;
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
