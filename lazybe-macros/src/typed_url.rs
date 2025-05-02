use proc_macro2::{Ident, TokenStream};
use quote::{TokenStreamExt, quote};
use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;
use syn::token::Paren;
use syn::{LitStr, Token, Type};

pub enum PathSegment {
    Static(LitStr),
    Dynamic { ident: Ident, ty: Type },
}

impl Parse for PathSegment {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        if input.peek(LitStr) {
            let path: LitStr = input.parse()?;
            return Ok(Self::Static(path));
        }

        if input.peek(Paren) {
            let content;
            syn::parenthesized!(content in input);
            let ident: Ident = content.parse()?;
            let ty: Type = content.parse()?;
            return Ok(Self::Dynamic { ident, ty });
        }

        Err(syn::Error::new(
            input.span(),
            "Expect path segment to be a string-literal or (ident:ty)",
        ))
    }
}

pub struct TypedUrlMeta {
    ident: Ident,
    path_segments: Vec<PathSegment>,
}

impl Parse for TypedUrlMeta {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let ident = input.parse()?;
        let _: Token![,] = input.parse()?;
        let path_segments = Punctuated::<PathSegment, Token![/]>::parse_separated_nonempty(input)?
            .into_iter()
            .collect();
        Ok(TypedUrlMeta { ident, path_segments })
    }
}

pub fn expand(url_meta: TypedUrlMeta) -> TokenStream {
    let mut ts = TokenStream::new();
    let ident = &url_meta.ident;
    ts.extend(quote! { pub struct #ident; });
    ts.extend(expand_axum_url(&url_meta));
    ts
}

pub fn expand_axum_url(url_meta: &TypedUrlMeta) -> TokenStream {
    let ident = &url_meta.ident;
    let axum_path = "/".to_string()
        + &url_meta
            .path_segments
            .iter()
            .map(|i| match i {
                PathSegment::Static(lit_str) => lit_str.value(),
                PathSegment::Dynamic { ident, .. } => format!("{{{}}}", ident.to_string()),
            })
            .collect::<Vec<_>>()
            .join("/");

    quote! {
        impl #ident {
            fn axum_url() -> &'static str {
                #axum_path
            }
        }
    }
}
