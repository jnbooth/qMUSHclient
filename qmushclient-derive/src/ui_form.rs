use std::iter;

use proc_macro::TokenStream;
use proc_macro2::{Literal, Span};
use quote::{quote, ToTokens};
use syn::*;

#[inline]
pub fn ui_form(attrs: TokenStream, input: TokenStream) -> TokenStream {
    let ui_file_path = parse_macro_input!(attrs as Literal);
    let input = parse_macro_input!(input as ItemStruct);

    let struct_name = &input.ident;

    let mut fields = input
        .fields
        .iter()
        .map(|f| f.ident.clone().expect("fields must be named"))
        .collect::<Vec<_>>();
    let first_field = fields.remove(0);
    let field_names = fields
        .iter()
        .map(|ident| ident.to_token_stream().to_string())
        .collect::<Vec<_>>();

    let qwidget_ty = Ident::new("QWidget", Span::call_site());

    let need_cast = match &input.fields.iter().next().unwrap().ty {
        Type::Path(expr) => !match &expr.path.segments.last().unwrap().arguments {
            PathArguments::AngleBracketed(expr) => match &expr.args.first() {
                Some(GenericArgument::Type(Type::Path(expr))) => {
                    expr.path.segments.last().unwrap().ident == qwidget_ty
                }
                _ => true,
            },
            _ => true,
        },
        _ => true,
    };

    let widget_cast = if need_cast {
        quote!(qt_core::QBox::from_q_ptr(
            widget.into_q_ptr().dynamic_cast()
        ))
    } else {
        quote!(widget)
    };

    let load = quote! {
        impl #struct_name {
            pub fn load<P: cpp_core::CastInto<cpp_core::Ptr<qt_widgets::QWidget>>>(parent: P) -> Self {
                unsafe {
                    let loader = qt_ui_tools::QUiLoader::new_0a();
                    loader.set_language_change_enabled(true);
                    let bytes = include_bytes!(#ui_file_path);
                    let widget = loader.load_bytes_with_parent(bytes, parent);
                    assert!(!widget.is_null(), "invalid ui file");

                    Self {
                        #(
                            #fields: widget.find_child(#field_names).unwrap(),
                        )*
                        #first_field: #widget_cast,
                    }
                }
            }
        }
    };

    let expanded: proc_macro2::TokenStream = iter::once(input.to_token_stream())
        .chain(iter::once(load))
        .collect();

    TokenStream::from(expanded)
}
