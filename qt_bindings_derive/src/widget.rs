use proc_macro::TokenStream;
use quote::{quote, ToTokens};
use syn::*;

#[inline]
pub fn derive_widget(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as ItemStruct);

    let name = input.ident;

    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    let first_field = input
        .fields
        .iter()
        .next()
        .expect("widget field missing")
        .ident
        .clone()
        .expect("fields must be named");

    let widget = if first_field == "ui" {
        quote!(ui.widget)
    } else {
        first_field.into_token_stream()
    };

    let expanded = quote! {
        impl #impl_generics Widget for #name #ty_generics #where_clause {
            fn widget(&self) -> cpp_core::Ptr<qt_widgets::QWidget> {
                unsafe {
                    use cpp_core::CastFrom;
                    cpp_core::Ptr::cast_from(&self.#widget)
                }
            }
        }

        impl #impl_generics cpp_core::StaticUpcast<qt_core::QObject> for #name #ty_generics #where_clause {
            unsafe fn static_upcast(ptr: cpp_core::Ptr<Self>) -> cpp_core::Ptr<qt_core::QObject> {
                use cpp_core::CastFrom;
                cpp_core::Ptr::cast_from(&ptr.#widget)
            }
        }

        impl #impl_generics std::ops::Drop for #name #ty_generics #where_clause {
            fn drop(&mut self) {
                unsafe {
                    if let Some(raw) = self.#widget.as_raw_ref() {
                        cpp_core::CppDeletable::delete(raw)
                    }
                }
            }
        }
    };

    TokenStream::from(expanded)
}
