use proc_macro2::TokenStream;
use quote::quote;
use syn::{Data, DataEnum, DeriveInput};

pub fn expand(input: DeriveInput) -> TokenStream {
    match &input.data {
        Data::Enum(data_enum) => expand_enum(&input, data_enum).unwrap_or_else(|e| e.into_compile_error()),
        Data::Struct(_) => {
            syn::Error::new_spanned(&input.ident, "Struct is not supported by the provided macro").into_compile_error()
        }
        Data::Union(_) => {
            syn::Error::new_spanned(&input.ident, "Union is not supported by the provided macro").into_compile_error()
        }
    }
}

fn expand_enum(input: &DeriveInput, data_enum: &DataEnum) -> syn::Result<TokenStream> {
    if let Some(variant) = data_enum.variants.iter().find(|v| !v.fields.is_empty()) {
        Err(syn::Error::new_spanned(
            &variant.ident,
            "Enum variant cannot contain fields",
        ))?
    }

    let ident = &input.ident;
    let variant_idents = data_enum.variants.iter().map(|v| v.ident.clone()).collect::<Vec<_>>();
    let to_string_match_arms = variant_idents.iter().map(|ident| {
        let ident_str = ident.to_string();
        quote! { Self::#ident => #ident_str.to_string() }
    });
    let from_str_match_arms = variant_idents.iter().map(|ident| {
        let ident_str = ident.to_string();
        quote! { #ident_str => Self::#ident }
    });

    Ok(quote! {
        impl std::fmt::Display for #ident {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                let s = match self {
                    #(#to_string_match_arms),*
                };
                write!(f, "{}", s)
            }
        }

        impl std::str::FromStr for #ident {
            type Err = String;
            fn from_str(s: &str) -> Result<Self, Self::Err> {
                let parsed = match s {
                    #(#from_str_match_arms),*,
                    s => Err(format!("Cannot parse enum value '{}' to type {}", s, stringify!(#ident)))?,
                };
                Ok(parsed)
            }
        }

        impl sea_query::Nullable for #ident {
            fn null() -> sea_query::Value {
                sea_query::Value::String(None)
            }
        }

        impl From<#ident> for sea_query::Value {
            fn from(value: #ident) -> Self {
                sea_query::Value::String(Some(value.to_string().into()))
            }
        }

        impl<Db> sqlx::Type<Db> for #ident
        where
            Db: sqlx::Database,
            String: sqlx::Type<Db>,
        {
            fn type_info() -> <Db as sqlx::Database>::TypeInfo {
                <String as sqlx::Type<Db>>::type_info()
            }
        }

        impl<'r, Db> sqlx::Decode<'r, Db> for #ident
        where
            Db: sqlx::Database,
            for<'s> String: sqlx::Decode<'s, Db>,
        {
            fn decode(value: <Db as sqlx::Database>::ValueRef<'r>) -> Result<Self, sqlx::error::BoxDynError> {
                let value_str = <String as sqlx::Decode<Db>>::decode(value)?;
                let parsed = <#ident as std::str::FromStr>::from_str(&value_str)?;
                Ok(parsed)
            }
        }
    })
}
