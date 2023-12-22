use proc_macro::TokenStream;
use quote::quote;
use syn::punctuated::Punctuated;
use syn::*;

struct ErrorVariant {
    ident: Ident,
    field: Option<Ident>,
    ty: Type,
}

struct OtherVariant {
    field: Ident,
    display: String,
}

enum VariantParse {
    AsError(ErrorVariant),
    AsOther(OtherVariant),
}

impl ErrorVariant {
    fn field(&self) -> proc_macro2::TokenStream {
        let ident = &self.ident;
        match self.field.as_ref() {
            Some(field) => quote! {
                Self::#ident{#field: e}
            },
            None => quote! {
                Self::#ident(e)
            },
        }
    }

    fn parse(value: Variant) -> Result<VariantParse> {
        let ident = value.ident;
        let mut fields = match value.fields {
            Fields::Named(fields) => fields.named,
            Fields::Unnamed(fields) => fields.unnamed,
            Fields::Unit => Punctuated::new(),
        };
        if let Some(field) = fields.pop() {
            if fields.is_empty() {
                let field = field.into_value();
                for attr in value.attrs {
                    if let Ok(Meta::List(metas)) = attr.parse_meta() {
                        if metas.path.is_ident("error") {
                            return Err(syn::Error::new_spanned(
                                metas,
                                "attribute unsupported for this field",
                            ));
                        }
                    }
                }
                return Ok(VariantParse::AsError(Self {
                    ident,
                    field: field.ident,
                    ty: field.ty,
                }));
            }
        };
        let mut display: Option<String> = None;

        for attr in value.attrs {
            if let Ok(Meta::List(mut metas)) = attr.parse_meta() {
                if metas.path.is_ident("error") && !metas.nested.is_empty() {
                    if metas.nested.len() > 1 {
                        return Err(syn::Error::new_spanned(metas, "unexpected arguments"));
                    }

                    let lit =
                        match metas.nested.pop().unwrap().into_value() {
                            NestedMeta::Meta(Meta::NameValue(MetaNameValue {
                                path, lit, ..
                            })) if path.is_ident("display") => lit,
                            other => {
                                return Err(syn::Error::new_spanned(other, "invalid argument"));
                            }
                        };

                    if display.is_some() {
                        return Err(syn::Error::new_spanned(metas, "duplicate attribute"));
                    }

                    match lit {
                        Lit::Str(lit) => display = Some(lit.value()),
                        lit => return Err(syn::Error::new_spanned(lit, "expected a string")),
                    }
                }
            }
        }
        Ok(VariantParse::AsOther(OtherVariant {
            display: display.unwrap_or_else(|| ident.to_string()),
            field: ident,
        }))
    }
}

#[inline]
pub fn derive_error(input: TokenStream) -> Result<TokenStream> {
    let input: ItemEnum = parse(input)?;

    let name = input.ident;
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    let mut variants: Vec<ErrorVariant> = Vec::new();
    let mut other: Vec<OtherVariant> = Vec::new();
    for variant in input.variants {
        match ErrorVariant::parse(variant)? {
            VariantParse::AsError(v) => variants.push(v),
            VariantParse::AsOther(v) => other.push(v),
        };
    }

    let source_fallthrough = if other.is_empty() {
        quote! {}
    } else {
        quote!(_ => None,)
    };

    let fmt_variants: proc_macro2::TokenStream = variants
        .iter()
        .map(|x| {
            let field = x.field();
            quote! {
                #field => std::fmt::Display::fmt(&e, f),
            }
        })
        .collect();

    let fmt_other: proc_macro2::TokenStream = other
        .into_iter()
        .map(|OtherVariant { display, field }| {
            quote! {
                Self::#field => f.write_str(#display),
            }
        })
        .collect();

    let source_variants: proc_macro2::TokenStream = variants
        .iter()
        .map(|x| {
            let field = x.field();
            quote! {
                #field => Some(e),
            }
        })
        .collect();

    let from_variants: proc_macro2::TokenStream = variants
        .iter()
        .map(|x| {
            let ty = &x.ty;
            let field = x.field();
            quote! {
                impl #impl_generics From<#ty> for #name #ty_generics #where_clause {
                    fn from(e: #ty) -> Self {
                        #field
                    }
                }
            }
        })
        .collect();

    let expanded = quote! {
        impl #impl_generics std::fmt::Display for #name #ty_generics #where_clause {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                match self {
                    #fmt_variants
                    #fmt_other
                }
            }
        }

        impl #impl_generics std::error::Error for #name #ty_generics #where_clause {
            fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
                match self {
                    #source_variants
                    #source_fallthrough
                }
            }
        }

        impl #impl_generics From<std::convert::Infallible> for #name #ty_generics #where_clause {
            fn from(e: std::convert::Infallible) -> Self {
                match e {}
            }
        }

        #from_variants
    };

    Ok(TokenStream::from(expanded))
}
