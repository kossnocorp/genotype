use proc_macro::TokenStream;
use quote::{quote, ToTokens};
use syn::{parse_macro_input, Lit};

pub fn macro_attribute(attr: TokenStream, input: TokenStream) -> TokenStream {
    let item = parse_macro_input!(input);

    let expanded = match item {
        syn::Item::Struct(item) => {
            let literal = parse_macro_input!(attr as Lit);

            let serde_code = match literal {
                Lit::Str(lit_str) => str_serde_code(lit_str.value(), item.ident.clone()),

                Lit::Bool(lit_bool) => bool_serde_code(lit_bool.value(), item.ident.clone()),

                Lit::Int(lit_int) => int_serde_code(lit_int.base10_digits(), item.ident.clone()),

                _ => panic!("The #[literal] attribute only supports string, bool or int literals"),
            };

            quote! {
                #item

                #serde_code
            }
        }

        _ => panic!("The #[literal] attribute can only be used with structs"),
    };

    TokenStream::from(expanded)
}

fn str_serde_code(literal: String, target: syn::Ident) -> proc_macro2::TokenStream {
    serde_code(
        &literal,
        &target,
        SerdeConsts {
            serialize: "serialize_str".into(),
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
            serialize: "serialize_bool".into(),
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
            serialize: "serialize_i64",
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

struct SerdeConsts {
    pub serialize: &'static str,
    pub deserialize: &'static str,
    pub visit_fns: Vec<proc_macro2::TokenStream>,
}

fn serde_code<L>(literal: &L, target: &syn::Ident, consts: SerdeConsts) -> proc_macro2::TokenStream
where
    L: ToTokens,
{
    let serialize = syn::Ident::new(&consts.serialize, target.span());
    let serialize_code = quote! {
        impl serde::Serialize for #target {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                serializer.#serialize(#literal)
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
