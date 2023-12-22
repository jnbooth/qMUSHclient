use proc_macro::TokenStream;

mod ui_form;
mod widget;

#[proc_macro_attribute]
pub fn ui_form(attrs: TokenStream, input: TokenStream) -> TokenStream {
    crate::ui_form::ui_form(attrs, input)
}

#[proc_macro_derive(Widget)]
pub fn derive_widget(input: TokenStream) -> TokenStream {
    crate::widget::derive_widget(input)
}
