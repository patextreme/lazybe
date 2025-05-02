use proc_macro2::{Ident, TokenStream};
use quote::quote;
use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;
use syn::token::Paren;
use syn::{LitStr, Token, Type};

pub enum PathSegment {
    Static(LitStr),
    Dynamic { ident: Ident, ty: Box<Type> },
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
            let _: Token![:] = content.parse()?;
            let ty: Type = content.parse()?;
            return Ok(Self::Dynamic { ident, ty: ty.into() });
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

impl TypedUrlMeta {
    fn dynamic_segments(&self) -> Vec<(Ident, Box<Type>)> {
        self.path_segments
            .iter()
            .filter_map(|i| match i {
                PathSegment::Dynamic { ident, ty } => Some((ident.clone(), ty.clone())),
                _ => None,
            })
            .collect()
    }
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
    ts.extend(expand_new_url(&url_meta));
    ts
}

fn expand_axum_url(url_meta: &TypedUrlMeta) -> TokenStream {
    let ident = &url_meta.ident;
    let str_segments = url_meta
        .path_segments
        .iter()
        .map(|i| match i {
            PathSegment::Static(lit_str) => lit_str.value(),
            PathSegment::Dynamic { ident, .. } => format!("{{{}}}", ident),
        })
        .map(|i| format!("/{}", i));

    quote! {
        impl #ident {
            const AXUM_URL: &str = concat!(#(#str_segments),*);
        }
    }
}

fn expand_new_url(url_meta: &TypedUrlMeta) -> TokenStream {
    let ident = &url_meta.ident;
    let fn_args = url_meta
        .dynamic_segments()
        .into_iter()
        .map(|(ident, ty)| quote! { #ident: #ty });
    let fmt_args = url_meta
        .dynamic_segments()
        .into_iter()
        .map(|(ident, _)| quote! { #ident });
    let str_segments = url_meta
        .path_segments
        .iter()
        .map(|i| match i {
            PathSegment::Static(lit_str) => lit_str.value(),
            PathSegment::Dynamic { .. } => "{}".to_string(),
        })
        .map(|i| format!("/{}", i));
    quote! {
        impl #ident {
            pub fn new_url(#(#fn_args),*) -> String {
                format!(
                    concat!(#(#str_segments),*),
                    #(#fmt_args),*
                )
            }
        }
    }
}
