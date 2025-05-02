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

pub struct TypedUriMeta {
    ident: Ident,
    path_segments: Vec<PathSegment>,
    query_param: Option<Type>,
}

impl TypedUriMeta {
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

impl Parse for TypedUriMeta {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let ident = input.parse()?;
        let _: Token![,] = input.parse()?;
        let path_segments = Punctuated::<PathSegment, Token![/]>::parse_separated_nonempty(input)?
            .into_iter()
            .collect();

        let mut query_param = None;
        if input.peek(Token![?]) {
            let _: Token![?] = input.parse()?;
            let ty: Type = input.parse()?;
            query_param = Some(ty);
        }

        Ok(TypedUriMeta {
            ident,
            path_segments,
            query_param,
        })
    }
}

pub fn expand(uri_meta: TypedUriMeta) -> TokenStream {
    let mut ts = TokenStream::new();
    ts.extend(expand_uri_struct(&uri_meta));
    ts.extend(expand_axum_url(&uri_meta));
    ts.extend(expand_new_uri(&uri_meta));
    ts
}

fn expand_uri_struct(uri_meta: &TypedUriMeta) -> TokenStream {
    let ident = &uri_meta.ident;
    let dyn_segments = uri_meta.dynamic_segments();

    if dyn_segments.is_empty() {
        quote! { pub struct #ident; }
    } else {
        let defs = dyn_segments.into_iter().map(|(ident, ty)| quote! { pub #ident: #ty });
        quote! {
            #[derive(serde::Deserialize)]
            pub struct #ident {
                #(#defs),*
            }
        }
    }
}

fn expand_axum_url(uri_meta: &TypedUriMeta) -> TokenStream {
    let ident = &uri_meta.ident;
    let str_segments = uri_meta
        .path_segments
        .iter()
        .map(|i| match i {
            PathSegment::Static(lit_str) => lit_str.value(),
            PathSegment::Dynamic { ident, .. } => format!("{{{}}}", ident),
        })
        .map(|i| format!("/{}", i));

    quote! {
        impl #ident {
            const AXUM_PATH: &str = concat!(#(#str_segments),*);
        }
    }
}

fn expand_new_uri(uri_meta: &TypedUriMeta) -> TokenStream {
    let ident = &uri_meta.ident;
    let fn_args = uri_meta
        .dynamic_segments()
        .into_iter()
        .map(|(ident, ty)| quote! { #ident: #ty })
        .chain(uri_meta.query_param.as_ref().map(|ty| quote! { query: #ty }));
    let fmt_args = uri_meta
        .dynamic_segments()
        .into_iter()
        .map(|(ident, _)| quote! { #ident });
    let str_segments = uri_meta
        .path_segments
        .iter()
        .map(|i| match i {
            PathSegment::Static(lit_str) => lit_str.value(),
            PathSegment::Dynamic { .. } => "{}".to_string(),
        })
        .map(|i| format!("/{}", i));

    let full_url_block = if uri_meta.query_param.is_some() {
        quote! {
            let query_str = lazybe::uri::to_query_string(&query).expect(&format!("Unable to serialize {} to query parameter", stringify!(#ident)));
            if query_str.is_empty() {
                base_url
            } else {
                base_url + "?" + &query_str
            }
        }
    } else {
        quote! { base_url }
    };

    quote! {
        impl #ident {
            /// Construct a relative URI from path segements and query params
            pub fn new_uri(#(#fn_args),*) -> String {
                let base_url = format!(
                    concat!(#(#str_segments),*),
                    #(#fmt_args),*
                );
                #full_url_block
            }
        }
    }
}
