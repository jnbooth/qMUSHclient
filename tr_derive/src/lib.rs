use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::quote;
use syn::*;

#[proc_macro_derive(TrContext)]
pub fn derive_trcontext(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as Item);
    let (name, generics) = match input {
        Item::Enum(expr) => (expr.ident, expr.generics),
        Item::Struct(expr) => (expr.ident, expr.generics),
        Item::Type(expr) => (expr.ident, expr.generics),
        Item::Union(expr) => (expr.ident, expr.generics),
        _ => panic!("expected `enum`, `struct`, `type`, or `union`"),
    };

    let mut name_str = name.to_string();
    name_str.push('\0');
    let name_bytes = LitByteStr::new(name_str.as_bytes(), Span::call_site());
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    let expanded = quote! {
        impl #impl_generics TrContext for #name #ty_generics #where_clause {
            const CLASS_NAME: &'static std::ffi::CStr = unsafe { std::ffi::CStr::from_bytes_with_nul_unchecked(#name_bytes as &[u8]) };
        }
    };

    TokenStream::from(expanded)
}
