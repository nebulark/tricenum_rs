use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, spanned::Spanned, AttrStyle, Data, DeriveInput, Error, Fields};

fn get_repr_size(input: &DeriveInput) -> Option<syn::Ident> {
    let mut repr_size: Option<syn::Ident> = None;

    for a in &input.attrs {
        if matches!(a.style, AttrStyle::Outer) {
            if a.path().is_ident("repr") {
                a.parse_nested_meta(|meta| {
                    if let Some(ident) = meta.path.get_ident() {
                        if ["u8", "u16", "u32", "u64"].iter().any(|s| ident == s) {
                            repr_size = Some(ident.clone());
                        }
                    }

                    Ok(())
                })
                .unwrap();
            }
        }
    }
    return repr_size;
}

#[proc_macro_derive(TrivialEnum)]
pub fn my_macro(input: TokenStream) -> TokenStream {
    // Parse the input tokens into a syntax tree
    let input = parse_macro_input!(input as DeriveInput);

    let Some(repr) = get_repr_size(&input) else {
        return Error::new(
            input.span(),
            "must have an explicit repr of unsigned integer, e.g. #[repr(u8)]",
        )
        .to_compile_error()
        .into();
    };

    let Data::Enum(enum_data) = &input.data else {
        return Error::new(input.span(), "must be an enum")
            .to_compile_error()
            .into();
    };

    for v in enum_data.variants.iter() {
        if !matches!(v.fields, Fields::Unit) {
            return Error::new(v.fields.span(), "Variant must not have a field")
                .to_compile_error()
                .into();
        }

        if let Some(_) = &v.discriminant {
            return Error::new(v.span(), "Variant must not have an explicit discriminant")
                .to_compile_error()
                .into();
        }
    }

    let enum_len = enum_data.variants.len();

    let ident = input.ident;

    let expanded = quote! {
        unsafe impl TrivialEnum for #ident {
            const ENUM_SIZE: usize = #enum_len;

            fn index(self) -> usize {
                self as usize
            }

            unsafe fn from_index_unchecked(val: usize) -> Self {
                unsafe { std::mem::transmute(val as #repr) }
            }
        }
    };

    TokenStream::from(expanded)
}
