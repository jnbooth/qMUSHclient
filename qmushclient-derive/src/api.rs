use std::iter;

use proc_macro::TokenStream;
use quote::{quote, ToTokens};
use syn::*;

pub fn api(attrs: TokenStream, input: TokenStream) -> TokenStream {
    parse_macro_input!(attrs as LitStr);
    input
}

fn is_lua(pat: &PatType) -> bool {
    let path = match &*pat.ty {
        Type::Path(path) => path,
        Type::Reference(tyref) => match &*tyref.elem {
            Type::Path(path) => path,
            _ => return false,
        },
        _ => return false,
    };
    match path.path.segments.last() {
        Some(segment) => segment.ident == "Lua",
        None => false,
    }
}

pub fn api_provider(_: TokenStream, input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as ItemImpl);
    if input.trait_.is_some() {
        panic!("Expected a bare impl, not a trait implementation.");
    }
    let ty = &input.self_ty;
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    let items: proc_macro2::TokenStream = input
        .items
        .iter()
        .filter_map(|item| {
            let me = match item {
                ImplItem::Method(me) => me,
                _ => return None,
            };
            let me_name = &me.sig.ident;
            let api_name = me.attrs.iter().find_map(|attr| {
                let meta = attr.parse_meta().ok()?;
                let list = match meta {
                    Meta::List(list) => list,
                    _ => return None,
                };
                if list.path.get_ident()? != "api" {
                    return None;
                }
                match list.nested.into_iter().next()? {
                    NestedMeta::Lit(Lit::Str(name)) => Some(name),
                    _ => None,
                }
            })?;

            let api_args: proc_macro2::TokenStream = me
                .sig
                .inputs
                .iter()
                .filter_map(|arg| {
                    let pat = match arg {
                        FnArg::Typed(pat) => pat,
                        FnArg::Receiver(..) => return None,
                    };
                    if is_lua(pat) {
                        None
                    } else {
                        let argname = &*pat.pat;
                        Some(quote! { #argname, })
                    }
                })
                .collect();
            let me_args: proc_macro2::TokenStream = me
                .sig
                .inputs
                .iter()
                .map(|arg| match arg {
                    FnArg::Receiver(..) => quote! { __this, },
                    FnArg::Typed(pat) => {
                        let argname = &*pat.pat;
                        if is_lua(pat) {
                            quote! { __lua, }
                        } else {
                            quote! { #argname, }
                        }
                    }
                })
                .collect();

            if matches!(me.sig.output, ReturnType::Default) {
                Some(quote! {
                    methods.add_method(#api_name, |__lua, __this, (#api_args)| {
                        #ty::#me_name(#me_args);
                        Ok(())
                    });
                })
            } else {
                Some(quote! {
                    methods.add_method(#api_name, |__lua, __this, (#api_args)| {
                        #ty::#me_name(#me_args)
                    });
                })
            }
        })
        .collect();

    let provide = quote! {
            pub fn #impl_generics provide_api #where_clause <'lua, M: UserDataMethods<'lua, #ty #ty_generics>>(methods: &mut M) {
                #items
        }
    };
    let expanded: proc_macro2::TokenStream = iter::once(input.to_token_stream())
        .chain(iter::once(provide))
        .collect();
    TokenStream::from(expanded)
}
