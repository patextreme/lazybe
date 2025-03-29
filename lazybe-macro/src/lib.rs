use proc_macro::TokenStream;
use syn::{DeriveInput, parse_macro_input};

mod dal_entity;
mod dal_enum;

#[proc_macro_derive(Dal, attributes(lazybe))]
pub fn derive_dal_entity(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    dal_entity::expand(input).into()
}

#[proc_macro_derive(DalEnum, attributes(lazybe))]
pub fn derive_eal_enum(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    dal_enum::expand(input).into()
}
