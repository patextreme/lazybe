use darling::FromMeta;
use proc_macro2::TokenStream;
use quote::quote;
use syn::Ident;

#[derive(Clone, FromMeta)]
pub enum CollectionApi {
    Default,
    Manual,
}

impl Default for CollectionApi {
    fn default() -> Self {
        Self::Default
    }
}

#[derive(Clone, FromMeta)]
pub enum ValidationHook {
    Default,
    Manual,
}

impl Default for ValidationHook {
    fn default() -> Self {
        Self::Default
    }
}

pub fn entity_route_trait_impl(entity: &Ident, base_url: &str) -> TokenStream {
    let get_path = format!("{}/{{id}}", base_url);
    let list_path = base_url.to_string();
    quote! {
        impl lazybe::router::Routable for #entity {
            fn entity_path() -> &'static str {
                #get_path
            }
            fn entity_collection_path() -> &'static str {
                #list_path
            }
        }
    }
}

pub fn entity_collection_api_trait_impl(
    entity: &Ident,
    collection_api: &CollectionApi,
    default_sort: Option<(&Ident, &Ident)>,
) -> TokenStream {
    let sort_expr = match default_sort {
        Some((pk_ident, sort_entity)) => quote! {
            lazybe::sort::Sort::new([#sort_entity::#pk_ident().asc()])
        },
        None => quote! { lazybe::sort::Sort::empty() },
    };
    match collection_api {
        CollectionApi::Manual => TokenStream::new(),
        CollectionApi::Default => quote! {
            impl lazybe::router::EntityCollectionApi for #entity {
                type Resp = Vec<Self>;
                type Query = ();

                fn page_response(page: lazybe::page::Page<Self>) -> Self::Resp {
                    page.data
                }

                fn page_input(_input: &Self::Query) -> Option<lazybe::page::PaginationInput> {
                    None
                }

                fn filter_input(_input: &Self::Query) -> lazybe::filter::Filter<Self> {
                    lazybe::filter::Filter::empty()
                }

                fn sort_input(_input: &Self::Query) -> lazybe::sort::Sort<Self> {
                    #sort_expr
                }
            }
        },
    }
}

pub fn entity_validation_hook_trait_impl(entity: &Ident, validation: &ValidationHook) -> TokenStream {
    match validation {
        ValidationHook::Manual => TokenStream::new(),
        ValidationHook::Default => {
            quote! { impl lazybe::router::ValidationHook for #entity {} }
        }
    }
}
