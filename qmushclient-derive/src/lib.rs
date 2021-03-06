use proc_macro::TokenStream;

mod api;
mod enums;
mod error;
mod widget;
mod trcontext;
mod ui_form;

#[proc_macro_attribute]
pub fn api(attrs: TokenStream, input: TokenStream) -> TokenStream {
    crate::api::api(attrs, input)
}
#[proc_macro_attribute]
pub fn api_provider(attrs: TokenStream, input: TokenStream) -> TokenStream {
    crate::api::api_provider(attrs, input)
}

#[proc_macro_attribute]
pub fn ui_form(attrs: TokenStream, input: TokenStream) -> TokenStream {
    crate::ui_form::ui_form(attrs, input)
}

#[proc_macro_derive(Widget)]
pub fn derive_widget(input: TokenStream) -> TokenStream {
    crate::widget::derive_widget(input)
}

#[proc_macro_derive(TrContext)]
pub fn derive_trcontext(input: TokenStream) -> TokenStream {
    crate::trcontext::derive_trcontext(input)
}

#[proc_macro_derive(Enum)]
pub fn derive_enum_variant_count(input: TokenStream) -> TokenStream {
    crate::enums::derive_enum_variant_count(input)
}

#[proc_macro_derive(Error, attributes(error))]
pub fn derive_error(input: TokenStream) -> TokenStream {
    match crate::error::derive_error(input) {
        Ok(output) => output,
        Err(err) => TokenStream::from(err.into_compile_error()),
    }
}
