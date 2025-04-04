use proc_macro::TokenStream;
use quote::{quote, ToTokens};
use syn::{parse_macro_input, Lit};

pub fn macro_attribute(attr: TokenStream, input: TokenStream) -> TokenStream {
    let item = parse_macro_input!(input);

    let expanded = match item {
        syn::Item::Struct(item) => {
            let attr_tokens = proc_macro2::TokenStream::from(attr.clone());
            let attr_str = attr_tokens.to_string().trim().to_string();

            let (serde_code, hash_code, debug_code) = if attr_str == "null" {
                let serde_code = null_serde_code(item.ident.clone());
                let hash_code = quote! {};
                let debug_code = debug_trait_code(&"null", &item.ident);

                (serde_code, hash_code, debug_code)
            } else {
                let literal = parse_macro_input!(attr as Lit);

                let (hasher_code, serde_code) = match &literal {
                    Lit::Str(lit_str) => (
                        std_hasher(&literal),
                        str_serde_code(lit_str.value(), item.ident.clone()),
                    ),

                    Lit::Bool(lit_bool) => (
                        std_hasher(&literal),
                        bool_serde_code(lit_bool.value(), item.ident.clone()),
                    ),

                    Lit::Int(lit_int) => (
                        std_hasher(&literal),
                        int_serde_code(lit_int.base10_digits(), item.ident.clone()),
                    ),

                    Lit::Float(lit_float) => {
                        let literal: f64 = lit_float
                            .base10_digits()
                            .parse()
                            .expect("Invalid f64 literal");
                        (
                            float_hasher(&literal),
                            float_serde_code(&literal, item.ident.clone()),
                        )
                    }

                    _ => panic!(
                    "The #[literal] attribute only supports string, bool, int or float literals"
                ),
                };

                let hash_code = hash_trait_code(hasher_code, &item.ident);

                let debug_code = debug_trait_code(&literal, &item.ident);

                (serde_code, hash_code, debug_code)
            };

            quote! {
                #[derive(Clone, Default, Eq, PartialEq)]
                #item

                #serde_code

                #hash_code

                #debug_code
            }
        }

        _ => panic!("The #[literal] attribute can only be used with structs"),
    };

    TokenStream::from(expanded)
}

fn null_serde_code(target: syn::Ident) -> proc_macro2::TokenStream {
    serde_code(
        &"null",
        &target,
        SerdeConsts {
            serialize_call: quote! { serialize_unit() },
            deserialize: "deserialize_unit".into(),
            visit_fns: vec![quote! {
                fn visit_unit<E>(self) -> Result<Self::Value, E>
                where
                    E: serde::de::Error,
                {
                    Ok(Null)
                }
            }],
        },
    )
}

fn str_serde_code(literal: String, target: syn::Ident) -> proc_macro2::TokenStream {
    serde_code(
        &literal,
        &target,
        SerdeConsts {
            serialize_call: quote! { serialize_str(#literal) },
            deserialize: "deserialize_str".into(),
            visit_fns: vec![serde_visit_code(
                &literal,
                &target,
                SerdeVisitConsts {
                    visit: "visit_str".into(),
                    visit_arg: quote! { &str },
                    visit_unexpected: "Str".into(),
                },
            )],
        },
    )
}

fn bool_serde_code(literal: bool, target: syn::Ident) -> proc_macro2::TokenStream {
    serde_code(
        &literal,
        &target,
        SerdeConsts {
            serialize_call: quote! { serialize_bool(#literal) },
            deserialize: "deserialize_bool".into(),
            visit_fns: vec![serde_visit_code(
                &literal,
                &target,
                SerdeVisitConsts {
                    visit: "visit_bool".into(),
                    visit_arg: quote! { bool },
                    visit_unexpected: "Bool".into(),
                },
            )],
        },
    )
}

fn int_serde_code(literal: &str, target: syn::Ident) -> proc_macro2::TokenStream {
    let literal: i64 = literal.parse().expect("Invalid i64 literal");
    serde_code(
        &literal,
        &target,
        SerdeConsts {
            serialize_call: quote! { serialize_i64(#literal) },
            deserialize: "deserialize_i64",
            visit_fns: vec![
                serde_visit_code(
                    &literal,
                    &target,
                    SerdeVisitConsts {
                        visit: "visit_u64",
                        visit_arg: quote! { u64 },
                        visit_unexpected: "Unsigned",
                    },
                ),
                serde_visit_code(
                    &literal,
                    &target,
                    SerdeVisitConsts {
                        visit: "visit_i64",
                        visit_arg: quote! { i64 },
                        visit_unexpected: "Signed",
                    },
                ),
            ],
        },
    )
}

fn float_serde_code(literal: &f64, target: syn::Ident) -> proc_macro2::TokenStream {
    serde_code(
        &literal,
        &target,
        SerdeConsts {
            serialize_call: quote! { serialize_f64(#literal) },
            deserialize: "deserialize_f64",
            visit_fns: vec![serde_visit_code(
                &literal,
                &target,
                SerdeVisitConsts {
                    visit: "visit_f64",
                    visit_arg: quote! { f64 },
                    visit_unexpected: "Float",
                },
            )],
        },
    )
}

struct SerdeConsts {
    pub serialize_call: proc_macro2::TokenStream,
    pub deserialize: &'static str,
    pub visit_fns: Vec<proc_macro2::TokenStream>,
}

fn serde_code<L>(literal: &L, target: &syn::Ident, consts: SerdeConsts) -> proc_macro2::TokenStream
where
    L: ToTokens,
{
    let serialize_call = consts.serialize_call;
    let serialize_code = quote! {
        impl serde::Serialize for #target {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                serializer.#serialize_call
            }
        }
    };

    let deserialize = syn::Ident::new(&consts.deserialize, target.span());
    let (visitor, visitor_ident) = serde_visitor_code(&target, literal, consts.visit_fns);
    let deserialize_code = quote! {
        impl<'de> serde::Deserialize<'de> for #target {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                deserializer.#deserialize(#visitor_ident)
            }
        }

        #visitor
    };

    quote! {
        #serialize_code

        #deserialize_code
    }
}

fn serde_visitor_code<L>(
    target: &syn::Ident,
    literal: &L,
    visit_fns: Vec<proc_macro2::TokenStream>,
) -> (proc_macro2::TokenStream, syn::Ident)
where
    L: ToTokens,
{
    let visitor = syn::Ident::new(&format!("{target}Visitor"), target.span());

    (
        quote! {
            pub struct #visitor;

            impl<'de> serde::de::Visitor<'de> for #visitor {
                type Value = #target;

                fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                    write!(formatter,  "a literal {}", #literal)
                }

                #(#visit_fns)*
            }
        },
        visitor,
    )
}

struct SerdeVisitConsts {
    pub visit: &'static str,
    pub visit_arg: proc_macro2::TokenStream,
    pub visit_unexpected: &'static str,
}

fn serde_visit_code<L>(
    literal: &L,
    target: &syn::Ident,
    consts: SerdeVisitConsts,
) -> proc_macro2::TokenStream
where
    L: ToTokens,
{
    let visit = syn::Ident::new(&consts.visit, target.span());
    let visit_arg = consts.visit_arg;
    let visit_unexpected = syn::Ident::new(&consts.visit_unexpected, target.span());

    quote! {
        fn #visit<E>(self, s: #visit_arg) -> Result<Self::Value, E>
        where
            E: serde::de::Error,
        {
            if s == #literal as #visit_arg {
                Ok(#target)
            } else {
                Err(serde::de::Error::invalid_value(
                    serde::de::Unexpected::#visit_unexpected(s.into()), &self
                ))
            }
        }
    }
}

fn std_hasher<L>(literal: &L) -> proc_macro2::TokenStream
where
    L: ToTokens,
{
    quote! { #literal.hash(state) }
}

fn float_hasher(literal: &f64) -> proc_macro2::TokenStream {
    quote! {
        let mut bits = #literal.to_bits();

        // Treat all NaN values the same
        if #literal.is_nan() {
            bits = f64::NAN.to_bits();
        } else if bits == (-0.0f64).to_bits() {
            // Normalize -0.0 to 0.0
            bits = 0.0f64.to_bits();
        }

        bits.hash(state);
    }
}

fn hash_trait_code(
    hasher: proc_macro2::TokenStream,
    target: &syn::Ident,
) -> proc_macro2::TokenStream {
    quote! {
        impl std::hash::Hash for #target {
            fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
                #hasher
            }
        }
    }
}

fn debug_trait_code<L>(literal: &L, target: &syn::Ident) -> proc_macro2::TokenStream
where
    L: ToTokens,
{
    quote! {
        impl std::fmt::Debug for #target {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{:?}", #literal)
            }
        }
    }
}
