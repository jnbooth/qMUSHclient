use proc_macro::TokenStream;
use quote::{quote, ToTokens};
use syn::*;

#[inline]
fn _parse_generic(ty: &Type) -> Option<&Type> {
    let segments = match ty {
        Type::Path(TypePath {
            path: Path { segments, .. },
            ..
        }) => Some(segments),
        _ => None,
    }?;
    let last = segments.last()?;
    let args = match &last.arguments {
        PathArguments::AngleBracketed(a) => Some(&a.args),
        _ => None,
    }?;
    let mut subtype = None;
    for arg in args {
        if let GenericArgument::Type(t) = arg {
            if subtype.is_some() {
                return None;
            }
            subtype = Some(t);
        }
    }
    subtype
}

#[inline]
pub fn derive_binding(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as ItemStruct);

    let name = input.ident;
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    let first_field = input
        .fields
        .into_iter()
        .next()
        .expect("inner field missing");

    let (constructor, field_name) = match first_field.ident {
        Some(field) => (quote!(Self { #field: bind }), field.to_token_stream()),
        None => (quote!(Self(bind)), quote!(0)),
    };
    let field_ty = first_field.ty;

    let expanded = quote! {
        impl #impl_generics Binding for #name #ty_generics #where_clause {
            type Bind = #field_ty;
            #[inline]
            fn into_raw(self) -> <Self as Binding>::Bind {
                self.#field_name
            }
        }
        impl #impl_generics From<#field_ty> for #name #ty_generics #where_clause {
            #[inline]
            fn from(bind: #field_ty) -> Self {
                #constructor
            }
        }
    };

    TokenStream::from(expanded)
}
