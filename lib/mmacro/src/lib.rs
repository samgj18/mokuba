extern crate proc_macro;
use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::quote;
use syn::{parse_macro_input, Data::Enum, DeriveInput};

/// Derive constructor for any struct where the fields are from the type of the struct fields
#[proc_macro_derive(ConstructorM)]
pub fn derive_constructor(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;
    let fields = match input.data {
        syn::Data::Struct(s) => Ok(s.fields),
        _ => Err("ConstructorM can only be used on structs"),
    };
    match fields {
        Ok(fields) => {
            let fields_types = fields.iter().clone().map(|f| f.ty.to_owned());
            let field_n = fields.iter().clone().map(|f| f.ident.to_owned());
            let field_p = field_n.clone();

            let gen = quote! {
                impl #name {
                    pub fn new(#(#field_n: #fields_types),*) -> #name {
                        #name {
                            #(#field_p),*
                        }
                    }
                }
            };

            TokenStream::from(gen)
        }
        Err(e) => TokenStream::from(syn::Error::new(Span::call_site(), e).to_compile_error()),
    }
}

/// Derive a Display macro to an Enum by using `&self` as the display string.
#[proc_macro_derive(DisplayM)]
pub fn derive_display(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;
    let gen = quote! {
        impl std::fmt::Display for #name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{:?}", self)
            }
        }
    };
    TokenStream::from(gen)
}

/// Derive a macro to match each variant of an Generic Enum to a string for valid sintax:
/// ```
/// #[derive(mmacro::VariantM)]
/// pub enum Test {
///     A,
///     B,
/// }
///
/// #[derive(mmacro::VariantM)]
/// pub enum C {
///    Three { a: f64, b: String },
/// }
///
/// #[derive(mmacro::VariantM)]
/// pub enum D {
///     Three(u8, bool),
/// }
/// ```
///
#[proc_macro_derive(VariantM)]
pub fn derive_enum_variants(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;

    let fields = match input.data {
        syn::Data::Enum(data) => Ok(data.variants),
        _ => Err("VariantM can only be used on enums"),
    };

    match fields {
        Ok(fields) => {
            let variants_with_fields = fields.into_iter().map(|f| {
                let variant_name = f.ident;
                let variant_fields = f.fields;
                match variant_fields {
                    syn::Fields::Unit => {
                        quote! {
                            #name::#variant_name => stringify!(#variant_name),
                        }
                    }
                    syn::Fields::Named(fields) => {
                        let fields = fields.named;
                        let fields_names = fields.iter().map(|f| f.ident.to_owned());
                        let fields_types = fields.iter().map(|f| f.ty.to_owned());

                        let fields_names_p = fields_names.clone();
                        let fields_types_p = fields_types.clone();

                        quote! {
                            #name::#variant_name { #(#fields_names: #fields_types),* } => stringify!(#variant_name { #(#fields_names_p: #fields_types_p),* }),
                        }
                    }
                    syn::Fields::Unnamed(fields) => {
                        let fields = fields.unnamed;
                        let fields_types = fields.iter().map(|f| f.ty.to_owned());
                        let fields_within_type = fields_types.clone().enumerate().map(|(i, _)| {
                            syn::Ident::new(&format!("f{}", i), Span::call_site())
                        });

                        let fields_types_p = fields_within_type.clone();

                        quote! {
                            #name::#variant_name (#(#fields_within_type),*) => stringify!(#variant_name { #(#fields_types_p),* }),
                        }
                    }
                }
            });

            let gen = quote! {
                impl #name {
                    pub fn variant(&self) -> &str {
                        match self {
                            #(#variants_with_fields)*
                        }
                    }
                }
            };

            TokenStream::from(gen)
        }
        Err(err) => TokenStream::from(syn::Error::new(Span::call_site(), err).to_compile_error()),
    }
}

/// Derive macro to create `parse` function, with type: `pub fn parse(input: &str) -> std::result::Result<#name, String>`
#[proc_macro_derive(ParserM)]
pub fn derive_parser(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;
    let fields = match input.data {
        Enum(data) => Ok(data.variants),

        _ => Err("VariantM can only be derived for enums"),
    };

    match fields {
        Ok(fields) => {
            let variants = fields.iter().map(|f| &f.ident).collect::<Vec<_>>();
            let variants_lower = variants
                .iter()
                .map(|v| v.to_string().to_lowercase())
                .collect::<Vec<_>>();

            let gen = quote! {
                impl #name {
                    pub fn parse(input: &str) -> std::result::Result<#name, String> {
                        match input.trim() {
                            #(#variants_lower => Ok(#name::#variants),)*
                            _ => Err(format!("Unknown input {}", input)
                            ),
                        }
                    }
                }
            };
            TokenStream::from(gen)
        }
        Err(err) => TokenStream::from(syn::Error::new(Span::call_site(), err).to_compile_error()),
    }
}

/// Derive macro to map from a struct that represents a custom error to a std::io::Error. You must have `std::io::{Error, ErrorKind}` in scope.
#[proc_macro_derive(ErrorM)]
pub fn derive_error(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;
    let fields = match input.data {
        syn::Data::Struct(s) => Ok(s.fields),
        _ => Err("ErrorM can only be used on structs"),
    };
    match fields {
        Ok(fields) => {
            let field_n = fields.iter().clone().map(|f| f.ident.to_owned());

            let gen = quote! {
                impl From<#name> for Error {
                    fn from(e: #name) -> Self {
                        let mut s = String::new();
                        #(s.push_str(&format!("{}: {:?} ", stringify!(#field_n), e.#field_n));)*
                        std::io::Error::new(std::io::ErrorKind::Other, s)
                    }
                }
            };

            TokenStream::from(gen)
        }
        Err(e) => TokenStream::from(syn::Error::new(Span::call_site(), e).to_compile_error()),
    }
}
