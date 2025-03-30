use proc_macro::TokenStream;
use syn::{DeriveInput, parse_macro_input};

mod dal_entity;
mod dal_enum;
mod dal_newtype;

#[proc_macro_derive(DalEntity, attributes(lazybe))]
pub fn derive_dal_entity(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    dal_entity::expand(input).into()
}

#[proc_macro_derive(DalEnum, attributes(lazybe))]
pub fn derive_dal_enum(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    dal_enum::expand(input).into()
}

#[proc_macro_derive(DalNewtype)]
pub fn derive_dal_newtype(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    dal_newtype::expand(input).into()
}
