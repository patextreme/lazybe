use convert_case::{Case, Casing};
use darling::{FromDeriveInput, FromField};
use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::{Data, DataStruct, DeriveInput, Field, Fields, FieldsNamed, Ident, Visibility};

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
    update_entity: Ident,
    filter_entity: Ident,
    sqlx_row_ident: Ident,
    sea_query_ident: Ident,
    primary_key: Field,
    created_at: Option<Field>,
    updated_at: Option<Field>,
    all_fields: Vec<(Field, EntityFieldAttr)>,
    user_defined_fields: Vec<(Field, EntityFieldAttr)>,
}

impl EntityMeta {
    fn primary_key_ident(&self) -> Ident {
        self.primary_key
            .ident
            .clone()
            .expect("Field of a primary key should be a named field")
    }

    fn primary_key_ident_pascal(&self) -> Ident {
        format_ident!("{}", self.primary_key_ident().to_string().to_case(Case::Pascal))
    }

    fn created_at_ident(&self) -> Option<Ident> {
        self.created_at.as_ref().and_then(|f| f.ident.as_ref()).cloned()
    }

    fn created_at_ident_pascal(&self) -> Option<Ident> {
        self.created_at_ident()
            .map(|i| format_ident!("{}", i.to_string().to_case(Case::Pascal)))
    }

    fn updated_at_ident(&self) -> Option<Ident> {
        self.updated_at.as_ref().and_then(|f| f.ident.as_ref()).cloned()
    }

    fn updated_at_ident_pascal(&self) -> Option<Ident> {
        self.updated_at_ident()
            .map(|i| format_ident!("{}", i.to_string().to_case(Case::Pascal)))
    }

    fn all_fields_ident(&self) -> Vec<Ident> {
        self.all_fields
            .iter()
            .flat_map(|(f, _)| f.ident.as_ref().cloned())
            .collect::<Vec<_>>()
    }

    fn all_fields_ident_pascal(&self) -> Vec<Ident> {
        self.all_fields_ident()
            .into_iter()
            .map(|i| format_ident!("{}", i.to_string().to_case(Case::Pascal)))
            .collect::<Vec<_>>()
    }

    fn user_defined_fields_ident(&self) -> Vec<Ident> {
        self.user_defined_fields
            .iter()
            .flat_map(|(f, _)| f.ident.as_ref().cloned())
            .collect::<Vec<_>>()
    }

    fn user_defined_fields_ident_pascal(&self) -> Vec<Ident> {
        self.user_defined_fields_ident()
            .into_iter()
            .map(|i| format_ident!("{}", i.to_string().to_case(Case::Pascal)))
            .collect::<Vec<_>>()
    }

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
            update_entity: format_ident!("Update{}", input.ident),
            filter_entity: format_ident!("{}Filter", input.ident),
            sqlx_row_ident: format_ident!("{}SqlxRow", input.ident),
            sea_query_ident: format_ident!("{}SeaQueryIdent", input.ident),
            primary_key: Self::detect_primary_key(&input.ident, &parsed_fields)?,
            created_at: Self::detect_created_at(&input.ident, &parsed_fields)?,
            updated_at: Self::detect_updated_at(&input.ident, &parsed_fields)?,
            all_fields: parsed_fields.to_vec(),
            user_defined_fields: parsed_fields
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
    let entity = &entity_meta.entity_ident;
    let entity_vis = &entity_meta.entity_vis;
    let entity_sea_query_ident = &entity_meta.sea_query_ident;
    let create_entity = &entity_meta.create_entity;
    let update_entity = &entity_meta.update_entity;
    let filter_entity = &entity_meta.filter_entity;
    let user_defined_field_defs = entity_meta.user_defined_fields.iter().map(|f| {
        let field_vis = &f.0.vis;
        let field_ident = f.0.ident.as_ref().unwrap();
        let field_ty = &f.0.ty;
        quote! { #field_vis #field_ident: #field_ty }
    });
    let update_entity_field_defs = entity_meta.user_defined_fields.iter().map(|f| {
        let field_vis = &f.0.vis;
        let field_ident = f.0.ident.as_ref().unwrap();
        let field_ty = &f.0.ty;
        quote! { #field_vis #field_ident: Option<#field_ty> }
    });
    let filter_method_defs = entity_meta
        .all_fields
        .iter()
        .map(|(f, _)| f)
        .zip(entity_meta.all_fields_ident_pascal())
        .map(|(f, field_ident_pascal)| {
            let field_ident = f.ident.as_ref().unwrap();
            let field_ty = &f.ty;
            quote! {
                pub fn #field_ident() -> lazybe::filter::FilterCol<#entity, #field_ty> {
                    lazybe::filter::FilterCol::new(#entity_sea_query_ident::#field_ident_pascal)
                }
            }
        });
    quote! {
        #[derive(Clone)]
        #entity_vis struct #create_entity {
            #(#user_defined_field_defs),*
        }
        #[derive(Default, Clone)]
        #entity_vis struct #update_entity {
            #(#update_entity_field_defs),*
        }
        #[derive(Clone)]
        #entity_vis struct #filter_entity;

        impl #filter_entity {
            #(#filter_method_defs)*
        }
    }
}

fn entity_row_impl(entity_meta: &EntityMeta) -> TokenStream {
    let entity = &entity_meta.entity_ident;
    let entity_vis = &entity_meta.entity_vis;
    let sqlx_row_ident = &entity_meta.sqlx_row_ident;
    let sea_query_ident = &entity_meta.sea_query_ident;
    let table_name = entity_meta.entity_attr.table.to_string();
    let all_field_idents = entity_meta.all_fields.iter().map(|(f, _)| {
        let ident_str = f.ident.as_ref().unwrap().to_string();
        let ident_pascal = format_ident!("{}", ident_str.to_case(Case::Pascal));
        quote! {
            #[iden = #ident_str]
            #ident_pascal
        }
    });
    let all_field_defs = entity_meta.all_fields.iter().map(|(f, _)| {
        let ident = f.ident.as_ref().unwrap();
        let ty = &f.ty;
        quote! { #ident: #ty }
    });
    let all_field_intos = entity_meta.all_fields.iter().map(|(f, _)| {
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
    let get_query_trait_impl = get_query_trait_impl(entity_meta);
    let list_query_trait_impl = list_query_trait_impl(entity_meta);
    let create_query_trait_impl = create_query_trait_impl(entity_meta);
    let update_query_trait_impl = update_query_trait_impl(entity_meta);
    let delete_query_trait_impl = delete_query_trait_impl(entity_meta);
    quote! {
        #get_query_trait_impl
        #list_query_trait_impl
        #create_query_trait_impl
        #update_query_trait_impl
        #delete_query_trait_impl
    }
}

fn create_query_trait_impl(entity_meta: &EntityMeta) -> TokenStream {
    let entity = &entity_meta.entity_ident;
    let create_entity = &entity_meta.create_entity;
    let sqlx_row_ident = &entity_meta.sqlx_row_ident;
    let sea_query_ident = &entity_meta.sea_query_ident;
    let user_defined_fields_ident = entity_meta.user_defined_fields_ident();
    let user_defined_fields_ident_pascal = entity_meta.user_defined_fields_ident_pascal();
    let now_value = Some(quote! { let now = sqlx::types::chrono::Utc::now(); })
        .filter(|_| entity_meta.created_at.is_some() || entity_meta.updated_at.is_some());
    let created_at_ident_pascal = entity_meta
        .created_at_ident_pascal()
        .map(|ident| quote! { #sea_query_ident::#ident })
        .into_iter();
    let updated_at_ident_pascal = entity_meta
        .updated_at_ident_pascal()
        .map(|ident| quote! { #sea_query_ident::#ident })
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
    quote! {
        impl lazybe::CreateQuery for #entity {
            type Create = #create_entity;
            type Row = #sqlx_row_ident;
            fn create_query(input: Self::Create) -> sea_query::InsertStatement {
                #now_value
                sea_query::Query::insert()
                    .into_table(#sea_query_ident::Table)
                    .columns([
                        #(#sea_query_ident::#user_defined_fields_ident_pascal,)*
                        #(#created_at_ident_pascal,)*
                        #(#updated_at_ident_pascal,)*
                    ])
                   .values_panic([
                        #(input.#user_defined_fields_ident.into(),)*
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
    let update_entity = &entity_meta.update_entity;
    let sqlx_row_ident = &entity_meta.sqlx_row_ident;
    let sea_query_ident = &entity_meta.sea_query_ident;
    let pk_ty = &entity_meta.primary_key.ty;
    let pk_ident_pascal = entity_meta.primary_key_ident_pascal();
    let now_value = Some(quote! { let now = sqlx::types::chrono::Utc::now(); })
        .filter(|_| entity_meta.created_at.is_some() || entity_meta.updated_at.is_some());
    let update_user_defined_fields = entity_meta
        .user_defined_fields_ident()
        .into_iter()
        .zip(entity_meta.user_defined_fields_ident_pascal())
        .map(|(ident, ident_pascal)| {
            quote! {
                if let Some(new_value) = input.#ident {
                    values.push((#sea_query_ident::#ident_pascal, new_value.into()));
                }
            }
        });
    let update_updated_at = entity_meta
        .updated_at_ident_pascal()
        .map(|ident| {
            quote! { values.push((#sea_query_ident::#ident, now.into())); }
        })
        .into_iter();
    quote! {
        impl lazybe::UpdateQuery for #entity {
            type Pk = #pk_ty;
            type Update = #update_entity;
            type Row = #sqlx_row_ident;
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
    let sqlx_row_ident = &entity_meta.sqlx_row_ident;
    let sea_query_ident = &entity_meta.sea_query_ident;
    let pk_ty = &entity_meta.primary_key.ty;
    let pk_ident_pascal = entity_meta.primary_key_ident_pascal();
    let all_field_idents_pascal = entity_meta.all_fields_ident_pascal();
    quote! {
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
    }
}

fn list_query_trait_impl(entity_meta: &EntityMeta) -> TokenStream {
    let entity = &entity_meta.entity_ident;
    let sqlx_row_ident = &entity_meta.sqlx_row_ident;
    let sea_query_ident = &entity_meta.sea_query_ident;
    let all_field_idents_pascal = entity_meta.all_fields_ident_pascal();
    quote! {
        impl lazybe::ListQuery for #entity {
            type Row = #sqlx_row_ident;
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
    let pk_ty = &entity_meta.primary_key.ty;
    let pk_ident_pascal = entity_meta.primary_key_ident_pascal();
    quote! {
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
