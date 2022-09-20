extern crate proc_macro;
use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::quote;
use syn::{parse_macro_input, Data::Enum, DeriveInput};

/// Derive macro for a struct that implements the MokubaError trait.
#[proc_macro_derive(MokubaErrorM)]
pub fn derive_mokuba_error(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;
    let gen = quote! {
        impl Display for #name {
            fn fmt(&self, f: &mut Formatter<'_>) -> Result {
                write!(f, "{}", self.code)
            }
        }

        impl #name {
            pub fn new(code: ErrorCode, cause: Option<&str>) -> #name {
                #name {
                    code,
                    cause: cause.map(|s| s.to_string()),
                }
            }
        }

        impl MokubaError<#name> for #name {
            fn description(&self) -> &str {
                &self.code.description_m()
            }

            fn cause(&self) -> Option<&str> {
                self.cause.as_deref()
            }
        }
    };
    TokenStream::from(gen)
}

/// Derive a Display macro to an Enum by using `&self` as the display string.
#[proc_macro_derive(DisplayM)]
pub fn derive_error_code(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;
    let gen = quote! {
        impl Display for #name {
            fn fmt(&self, f: &mut Formatter<'_>) -> Result {
                write!(f, "{:?}", self)
            }
        }
    };
    TokenStream::from(gen)
}

/// Derive a macro to match each variant of an Generic Enum to a string.
#[proc_macro_derive(VariantM)]
pub fn derive_enum_variants(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;
    let fields = match input.data {
        Enum(data) => Ok(data.variants),

        _ => Result::Err("VariantM can only be derived for enums"),
    };

    match fields {
        Ok(fields) => {
            let variants = fields.iter().map(|f| &f.ident).collect::<Vec<_>>();

            let gen = quote! {
                impl #name {
                    pub fn description_m(&self) -> &str {
                        match self {
                            #(#name::#variants => stringify!(#variants),)*
                        }
                    }
                }
            };
            TokenStream::from(gen)
        }
        Err(err) => TokenStream::from(syn::Error::new(Span::call_site(), err).to_compile_error()),
    }
}
