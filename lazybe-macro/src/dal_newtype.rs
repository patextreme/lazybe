use proc_macro2::TokenStream;
use quote::quote;
use syn::{Data, DataStruct, DeriveInput, Fields};

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
        Fields::Unnamed(fields_unnamed) => {
            if fields_unnamed.unnamed.len() != 1 {
                Err(syn::Error::new_spanned(
                    &input.ident,
                    "Newtype must contain exactly 1 unnamed field",
                ))?
            }
            let field = fields_unnamed.unnamed.iter().next().unwrap();
            let newtype = &input.ident;
            let inner_ty = &field.ty;
            Ok(quote! {
                impl<'r, Db> sqlx::Decode<'r, Db> for #newtype
                where
                    Db: sqlx::Database,
                    for<'s> #inner_ty: sqlx::Decode<'s, Db>,
                {
                    fn decode(value: <Db as sqlx::Database>::ValueRef<'r>) -> Result<Self, sqlx::error::BoxDynError> {
                        let inner_value = <#inner_ty as sqlx::Decode<Db>>::decode(value)?;
                        Ok(#newtype(inner_value))
                    }
                }

                impl<Db> sqlx::Type<Db> for #newtype
                where
                    Db: sqlx::Database,
                    #inner_ty: sqlx::Type<Db>,
                {
                    fn type_info() -> <Db as sqlx::Database>::TypeInfo {
                        <#inner_ty as sqlx::Type<Db>>::type_info()
                    }
                }

                impl From<#newtype> for sea_query::Value {
                    fn from(value: #newtype) -> Self {
                        value.0.into()
                    }
                }

                impl sea_query::Nullable for #newtype {
                    fn null() -> sea_query::Value {
                        <#inner_ty as sea_query::Nullable>::null()
                    }
                }
            })
        }
        Fields::Named(_) => Err(syn::Error::new_spanned(
            &input.ident,
            "Named struct is not supported by the provided macro",
        ))?,
        Fields::Unit => Err(syn::Error::new_spanned(
            &input.ident,
            "Unit struct is not supported by the provided macro",
        ))?,
    }
}
