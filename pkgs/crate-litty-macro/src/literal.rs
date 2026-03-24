use proc_macro::TokenStream;
use quote::{ToTokens, quote};
use syn::{Ident, Lit, parse_macro_input};

#[derive(Clone, Copy)]
enum StructSerdeMode {
    Both,
    SerializeOnly,
    DeserializeOnly,
}

pub fn macro_attribute(attr: TokenStream, input: TokenStream) -> TokenStream {
    macro_attribute_with_mode(attr, input, StructSerdeMode::Both)
}

pub fn macro_attribute_serialize_literal(attr: TokenStream, input: TokenStream) -> TokenStream {
    macro_attribute_with_mode(attr, input, StructSerdeMode::SerializeOnly)
}

pub fn macro_attribute_deserialize_literal(attr: TokenStream, input: TokenStream) -> TokenStream {
    macro_attribute_with_mode(attr, input, StructSerdeMode::DeserializeOnly)
}

fn macro_attribute_with_mode(
    attr: TokenStream,
    input: TokenStream,
    serde_mode: StructSerdeMode,
) -> TokenStream {
    let item = parse_macro_input!(input);

    let syn::Item::Struct(item) = item else {
        panic!(
            "The #[literal], #[serialize_literal], and #[deserialize_literal] attributes can only be used with unit structs"
        )
    };

    if !matches!(item.fields, syn::Fields::Unit) {
        panic!(
            "The #[literal], #[serialize_literal], and #[deserialize_literal] attributes can only be used with unit structs"
        )
    }

    let attr_tokens = proc_macro2::TokenStream::from(attr.clone());
    let attr_str = attr_tokens.to_string().trim().to_string();

    let traits_code = if attr_str == "null" {
        struct_lit_trait_code(
            LitDef {
                literal_type: quote! { () },
                literal: quote! { () },
                trait_ident: quote! { LitNull },
                struct_ident: &item.ident,
            },
            serde_mode,
        )
    } else {
        lit_trait_code(&item.ident, &parse_macro_input!(attr as Lit), serde_mode)
    };

    let expanded = quote! {
        #[derive(Clone, Default, Eq, PartialEq)]
        #item

        #traits_code
    };

    TokenStream::from(expanded)
}

fn lit_trait_code(
    struct_ident: &Ident,
    lit: &Lit,
    serde_mode: StructSerdeMode,
) -> proc_macro2::TokenStream {
    let trait_ident = lit_trait_ident(lit);
    match lit {
        Lit::Str(lit_str) => struct_lit_trait_code(
            LitDef {
                literal_type: quote! { &'static str },
                literal: lit_str.value(),
                trait_ident,
                struct_ident,
            },
            serde_mode,
        ),

        Lit::Bool(lit_bool) => struct_lit_trait_code(
            LitDef {
                literal_type: quote! { bool },
                literal: lit_bool.value(),
                trait_ident,
                struct_ident,
            },
            serde_mode,
        ),

        Lit::Int(lit_int) => struct_lit_trait_code(
            LitDef {
                literal_type: quote! { i64 },
                literal: lit_int
                    .base10_digits()
                    .parse::<i64>()
                    .expect("Invalid i64 literal"),
                trait_ident,
                struct_ident,
            },
            serde_mode,
        ),

        Lit::Float(lit_float) => struct_lit_trait_code(
            LitDef {
                literal_type: quote! { f64 },
                literal: lit_float
                    .base10_digits()
                    .parse::<f64>()
                    .expect("Invalid f64 literal"),
                trait_ident,
                struct_ident,
            },
            serde_mode,
        ),

        _ => panic!("The #[literal] attribute only supports string, bool, int or float literals"),
    }
}

fn lit_trait_ident(lit: &Lit) -> proc_macro2::TokenStream {
    match lit {
        Lit::Str(_) => quote! { LitStr },
        Lit::Bool(_) => quote! { LitBool },
        Lit::Int(_) => quote! { LitInt },
        Lit::Float(_) => quote! { LitFloat },
        _ => panic!("The #[literal] attribute only supports string, bool, int or float literals"),
    }
}

struct LitDef<'a, L: ToTokens> {
    literal_type: proc_macro2::TokenStream,
    literal: L,
    trait_ident: proc_macro2::TokenStream,
    struct_ident: &'a syn::Ident,
}

fn struct_lit_trait_code<L>(def: LitDef<L>, serde_mode: StructSerdeMode) -> proc_macro2::TokenStream
where
    L: ToTokens,
{
    let LitDef {
        literal_type,
        literal,
        trait_ident,
        struct_ident,
    } = def;

    let serde_impl = match serde_mode {
        StructSerdeMode::Both => quote! {
            impl serde::Serialize for #struct_ident {
                fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
                where
                    S: serde::Serializer,
                {
                    <Self as litty::#trait_ident>::lit_serialize(serializer)
                }
            }

            impl<'de> serde::Deserialize<'de> for #struct_ident {
                fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
                where
                    D: serde::Deserializer<'de>,
                {
                    use litty::#trait_ident;
                    Self::lit_deserialize(deserializer)?;
                    Ok(Self)
                }
            }
        },
        StructSerdeMode::SerializeOnly => quote! {
            impl serde::Serialize for #struct_ident {
                fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
                where
                    S: serde::Serializer,
                {
                    <Self as litty::#trait_ident>::lit_serialize(serializer)
                }
            }
        },
        StructSerdeMode::DeserializeOnly => quote! {
            impl<'de> serde::Deserialize<'de> for #struct_ident {
                fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
                where
                    D: serde::Deserializer<'de>,
                {
                    use litty::#trait_ident;
                    Self::lit_deserialize(deserializer)?;
                    Ok(Self)
                }
            }
        },
    };

    quote! {
    impl litty::#trait_ident for #struct_ident {
        const LIT: #literal_type = #literal;
    }

    #serde_impl

    impl std::hash::Hash for #struct_ident {
        fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
            use litty::#trait_ident;
            Self::lit_hash(state)
        }
    }

    impl std::fmt::Debug for #struct_ident {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            use litty::#trait_ident;
            Self::lit_fmt(f)
        }
    }
    }
}
