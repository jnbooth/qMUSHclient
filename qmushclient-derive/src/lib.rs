use proc_macro::TokenStream;

mod binding;
mod enums;
mod rwidget;
mod trcontext;
mod ui_form;

#[proc_macro_attribute]
pub fn ui_form(attrs: TokenStream, input: TokenStream) -> TokenStream {
    crate::ui_form::ui_form(attrs, input)
}

#[proc_macro_derive(RWidget)]
pub fn derive_rwidget(input: TokenStream) -> TokenStream {
    crate::rwidget::derive_rwidget(input)
}

#[proc_macro_derive(TrContext)]
pub fn derive_trcontext(input: TokenStream) -> TokenStream {
    crate::trcontext::derive_trcontext(input)
}

#[proc_macro_derive(Binding)]
pub fn derive_binding(input: TokenStream) -> TokenStream {
    crate::binding::derive_binding(input)
}

#[proc_macro_derive(Enum)]
pub fn derive_enum_variant_count(input: TokenStream) -> TokenStream {
    crate::enums::derive_enum_variant_count(input)
}
