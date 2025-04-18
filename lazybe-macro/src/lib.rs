use proc_macro::TokenStream;
use syn::{DeriveInput, parse_macro_input};

mod common;
mod entity;
mod entity_endpoint;
mod r#enum;
mod newtype;

#[proc_macro_derive(Entity, attributes(lazybe))]
pub fn derive_entity(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    entity::expand(input).into()
}

#[proc_macro_derive(Enum, attributes(lazybe))]
pub fn derive_enum(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    r#enum::expand(input).into()
}

#[proc_macro_derive(Newtype)]
pub fn derive_newtype(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    newtype::expand(input).into()
}

#[proc_macro_derive(EntityEndpoint, attributes(lazybe))]
pub fn derive_entity_endpoint(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    entity_endpoint::expand(input).into()
}
