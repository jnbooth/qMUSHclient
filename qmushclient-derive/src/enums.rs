use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::quote;
use syn::*;

#[inline]
pub fn derive_enum_variant_count(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as ItemEnum);

    let name = input.ident;
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    if input.variants.is_empty() {
        panic!("Type must not be empty");
    }

    if input.variants.iter().any(|x| x.discriminant.is_some()) {
        panic!("Manual discriminants are unsupported");
    }

    let size = input.variants.len();

    let rep = if size <= 8 {
        quote!(u8)
    } else if size <= 16 {
        quote!(u16)
    } else if size <= 32 {
        quote!(u32)
    } else if size <= 64 {
        quote!(u64)
    } else if size <= 128 {
        quote!(u128)
    } else {
        panic!("Too many variants")
    };

    let idx = input
        .attrs
        .iter()
        .map(Attribute::parse_meta)
        .filter_map(Result::ok)
        .filter(|x| x.path().is_ident("repr"))
        .filter_map(|x| match x {
            Meta::List(meta) => Some(meta.nested),
            _ => None,
        })
        .flat_map(IntoIterator::into_iter)
        .filter_map(|x| match x {
            NestedMeta::Meta(Meta::Path(path)) => Some(path.segments),
            _ => None,
        })
        .flat_map(IntoIterator::into_iter)
        .map(|x| x.ident)
        .next()
        .unwrap_or_else(|| Ident::new("u8", Span::call_site()));

    let min_bound = &input.variants.first().unwrap().ident;
    let max_bound = &input.variants.last().unwrap().ident;

    let to_strs: proc_macro2::TokenStream = input
        .variants
        .iter()
        .map(|x| {
            let ident = &x.ident;
            let tostr = x.ident.to_string();
            quote!(#name::#ident => #tostr,)
        })
        .collect();

    let from_strs: proc_macro2::TokenStream = input
        .variants
        .iter()
        .map(|x| {
            let fromstr = x.ident.to_string();
            let ident = &x.ident;
            quote!(#fromstr => Some(#name::#ident),)
        })
        .collect();

    let from_strs_ci: proc_macro2::TokenStream = input
        .variants
        .iter()
        .map(|x| {
            let fromstr = x.ident.to_string().to_lowercase();
            let ident = &x.ident;
            quote!(#fromstr => Some(#name::#ident),)
        })
        .collect();

    let expanded = quote! {
        impl #impl_generics Enum for #name #ty_generics #where_clause {
            type Rep = #rep;
            const SIZE: usize = #size;
            const MIN: Self = #name::#min_bound;
            const MAX: Self = #name::#max_bound;

            fn succ(self) -> Option<Self> {
                if self == #name::#max_bound {
                    None
                } else {
                    Some(unsafe { std::mem::transmute(self as #idx + 1) })
                }
            }

            fn pred(self) -> Option<Self> {
                if self == #name::#min_bound {
                    None
                } else {
                    Some(unsafe { std::mem::transmute(self as #idx - 1) })
                }
            }

            fn bit(self) -> Self::Rep {
                1 << (self as #idx)
            }

            fn index(self) -> usize {
                self as usize
            }

            fn from_index(i: usize) -> Option<Self> {
                if i < #size {
                    Some(unsafe { std::mem::transmute(i as #idx) })
                } else {
                    None
                }
            }
        }

        impl #impl_generics #name #ty_generics #where_clause {
            pub const fn bit(self) -> #idx {
                1 << (self as #idx)
            }

            pub const fn to_str(self) -> &'static str {
                match self {
                    #to_strs
                }
            }
            pub fn parse(s: &str) -> Option<Self> {
                match s {
                    #from_strs
                    _ => None,
                }
            }
            pub fn parse_ci(s: &str) -> Option<Self> {
                match s.to_lowercase().as_str() {
                    #from_strs_ci
                    _ => None,
                }
            }
        }
    };
    TokenStream::from(expanded)
}
