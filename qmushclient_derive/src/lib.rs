use proc_macro::TokenStream;

mod api;
mod error;

#[proc_macro_attribute]
pub fn api(attrs: TokenStream, input: TokenStream) -> TokenStream {
    crate::api::api(attrs, input)
}
#[proc_macro_attribute]
pub fn api_provider(attrs: TokenStream, input: TokenStream) -> TokenStream {
    crate::api::api_provider(attrs, input)
}

#[proc_macro_derive(Error, attributes(error))]
pub fn derive_error(input: TokenStream) -> TokenStream {
    match crate::error::derive_error(input) {
        Ok(output) => output,
        Err(err) => TokenStream::from(err.into_compile_error()),
    }
}
