use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Lit};

#[proc_macro_attribute]
pub fn literal(attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item);

    let expanded = match input {
        syn::Item::Struct(input) => {
            let literal = parse_macro_input!(attr as Lit);

            let serde_code = match literal {
                Lit::Str(lit_str) => gen_literal_serde_str(lit_str.value(), input.ident.clone()),

                Lit::Bool(lit_bool) => {
                    gen_literal_serde_bool(lit_bool.value(), input.ident.clone())
                }

                _ => panic!("The #[literal] attribute only supports string or bool literals"),
            };

            quote! {
                #input

                #serde_code
            }
        }

        syn::Item::Enum(input) => {
            // [TODO]
            quote! {
                #input
            }
        }

        _ => panic!("The #[literal] attribute can only be used with structs or enums"),
    };

    TokenStream::from(expanded)
}

fn gen_literal_serde_str(literal_str: String, ident: syn::Ident) -> proc_macro2::TokenStream {
    let visitor_ident = syn::Ident::new(&format!("{ident}Visitor"), ident.span());

    quote! {
        impl serde::Serialize for #ident {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                serializer.serialize_str(#literal_str)
            }
        }

        impl<'de> serde::Deserialize<'de> for #ident {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                deserializer.deserialize_str(#visitor_ident)
            }
        }

        pub struct #visitor_ident;

        impl<'de> serde::de::Visitor<'de> for #visitor_ident {
            type Value = #ident;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                write!(formatter, r#"a literal str \"{}\""#, #literal_str)
            }

            fn visit_str<E>(self, s: &str) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                if s == #literal_str {
                    Ok(#ident)
                } else {
                    Err(serde::de::Error::invalid_value(
                        serde::de::Unexpected::Str(s), &self
                    ))
                }
            }
        }
    }
}

fn gen_literal_serde_bool(literal: bool, ident: syn::Ident) -> proc_macro2::TokenStream {
    let visitor_ident = syn::Ident::new(&format!("{ident}Visitor"), ident.span());

    quote! {
        impl serde::Serialize for #ident {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                serializer.serialize_bool(#literal)
            }
        }

        impl<'de> serde::Deserialize<'de> for #ident {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                deserializer.deserialize_bool(#visitor_ident)
            }
        }

        pub struct #visitor_ident;

        impl<'de> serde::de::Visitor<'de> for #visitor_ident {
            type Value = #ident;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                write!(formatter, "a literal bool {}", #literal)
            }

            fn visit_bool<E>(self, b: bool) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                if b == #literal {
                    Ok(#ident)
                } else {
                    Err(serde::de::Error::invalid_value(
                        serde::de::Unexpected::Bool(b), &self
                    ))
                }
            }
        }
    }
}
