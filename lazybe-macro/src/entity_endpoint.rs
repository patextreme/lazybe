use darling::FromDeriveInput;
use proc_macro2::TokenStream;
use quote::quote;
use syn::{Data, DataStruct, DeriveInput, Fields, FieldsNamed, Ident, Type};

use crate::common::{self, CollectionApi, ValidationHook};

#[derive(Clone, FromDeriveInput)]
#[darling(attributes(lazybe))]
struct EntityAttr {
    endpoint: String,
    #[darling(default)]
    collection_api: CollectionApi,
    #[darling(default)]
    validation: ValidationHook,
    #[darling(default)]
    pk_ty: Option<Type>,
    #[darling(default)]
    create_ty: Option<Type>,
    #[darling(default)]
    update_ty: Option<Type>,
    #[darling(default)]
    replace_ty: Option<Type>,
}

pub struct EndpointMeta {
    entity_ident: Ident,
    attr: EntityAttr,
}

impl EndpointMeta {
    fn try_parse(input: &DeriveInput, _: &FieldsNamed) -> syn::Result<Self> {
        Ok(EndpointMeta {
            entity_ident: input.ident.clone(),
            attr: EntityAttr::from_derive_input(input)?,
        })
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
            let endpoint_meta = EndpointMeta::try_parse(input, fields_named)?;
            let entity_entity_trait_impl = entity_entity_trait_impl(&endpoint_meta);
            let entity_route_trait_impl =
                common::entity_route_trait_impl(&endpoint_meta.entity_ident, &endpoint_meta.attr.endpoint);
            let entity_collection_api_trait_impl = common::entity_collection_api_trait_impl(
                &endpoint_meta.entity_ident,
                &endpoint_meta.attr.collection_api,
                None,
            );
            let entity_validation_hook_trait_impl =
                common::entity_validation_hook_trait_impl(&endpoint_meta.entity_ident, &endpoint_meta.attr.validation);
            Ok(quote! {
                #entity_entity_trait_impl
                #entity_route_trait_impl
                #entity_collection_api_trait_impl
                #entity_validation_hook_trait_impl
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

fn entity_entity_trait_impl(endpoint_meta: &EndpointMeta) -> TokenStream {
    let entity = &endpoint_meta.entity_ident;
    let entity_str = entity.to_string();
    let pk_ty = &endpoint_meta
        .attr
        .pk_ty
        .as_ref()
        .map(|t| quote! { #t })
        .unwrap_or(quote! { String });
    let create_entity = &endpoint_meta
        .attr
        .create_ty
        .as_ref()
        .map(|t| quote! { #t })
        .unwrap_or(quote! { () });
    let update_entity = &endpoint_meta
        .attr
        .update_ty
        .as_ref()
        .map(|t| quote! { #t })
        .unwrap_or(quote! { () });
    let replace_entity = &endpoint_meta
        .attr
        .replace_ty
        .as_ref()
        .map(|t| quote! { #t })
        .unwrap_or(quote! { () });
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
    }
}
