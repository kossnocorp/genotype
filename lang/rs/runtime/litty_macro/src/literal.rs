use proc_macro::TokenStream;
use quote::{quote, ToTokens};
use syn::{parse_macro_input, Lit};

pub fn macro_attribute(attr: TokenStream, input: TokenStream) -> TokenStream {
    let item = parse_macro_input!(input);

    let expanded = match item {
        syn::Item::Struct(item) => {
            let attr_tokens = proc_macro2::TokenStream::from(attr.clone());
            let attr_str = attr_tokens.to_string().trim().to_string();

            let traits_code = if attr_str == "null" {
                struct_lit_trait_code(LitDef {
                    literal_type: quote! { () },
                    literal: quote! { () },
                    trait_ident: quote! { LitNull },
                    struct_ident: &item.ident,
                })
            } else {
                match &parse_macro_input!(attr as Lit) {
                    Lit::Str(lit_str) => struct_lit_trait_code(LitDef {
                        literal_type: quote! { &'static str },
                        literal: lit_str.value(),
                        trait_ident: quote! { LitStr },
                        struct_ident: &item.ident,
                    }),

                    Lit::Bool(lit_bool) => struct_lit_trait_code(LitDef {
                        literal_type: quote! { bool },
                        literal: lit_bool.value(),
                        trait_ident: quote! { LitBool },
                        struct_ident: &item.ident,
                    }),

                    Lit::Int(lit_int) => struct_lit_trait_code(LitDef {
                        literal_type: quote! { i64 },
                        literal: lit_int
                            .base10_digits()
                            .parse::<i64>()
                            .expect("Invalid i64 literal"),
                        trait_ident: quote! { LitInt },
                        struct_ident: &item.ident,
                    }),

                    Lit::Float(lit_float) => struct_lit_trait_code(LitDef {
                        literal_type: quote! { f64 },
                        literal: lit_float
                            .base10_digits()
                            .parse::<f64>()
                            .expect("Invalid f64 literal"),
                        trait_ident: quote! { LitFloat },
                        struct_ident: &item.ident,
                    }),

                    _ => panic!(
                    "The #[literal] attribute only supports string, bool, int or float literals"
                ),
                }
            };

            quote! {
                #[derive(Clone, Default, Eq, PartialEq)]
                #item

                #traits_code
            }
        }

        syn::Item::Enum(mut item) => {
            // Check if there're any literal variants
            let has_variant_attrs = item.variants.iter().any(|v| {
                v.attrs
                    .iter()
                    .any(|a| a.path().segments.iter().any(|s| s.ident == "literal"))
            });

            if has_variant_attrs {
                // Extract literals from variant attributes
                let mut variant_literals = Vec::new();

                for variant in &mut item.variants {
                    // Find literal attributes
                    let mut found_literal = None;
                    let literal_attrs: Vec<_> = variant
                        .attrs
                        .iter()
                        .enumerate()
                        .filter(|(_, a)| a.path().segments.iter().any(|s| s.ident == "literal"))
                        .collect();

                    if let Some((_, attr)) = literal_attrs.first() {
                        // Parse the literal from the attribute
                        if let Ok(meta) = attr.meta.require_list() {
                            if let Ok(lit) = syn::parse2::<Lit>(meta.tokens.clone()) {
                                found_literal = Some(lit);
                            }
                        }

                        // Store the variant name and its literal value
                        if let Some(lit) = found_literal.clone() {
                            variant_literals.push((variant.ident.clone(), lit));
                        }
                    }

                    // Remove literal attributes from variants (from end to avoid invalidating indices)
                    let indices: Vec<_> = literal_attrs.iter().map(|(i, _)| *i).collect();
                    for &idx in indices.iter().rev() {
                        variant.attrs.remove(idx);
                    }
                }

                // Generate code for enum with literal variants
                let enum_name = &item.ident;

                // Generate serialization impl
                let serialize_match_arms = variant_literals.iter().map(|(variant_name, literal)| {
                    match literal {
                        Lit::Str(s) => {
                            let value = s.value();
                            quote! {
                                #enum_name::#variant_name => serializer.serialize_str(#value)
                            }
                        },
                        Lit::Bool(b) => {
                            let value = b.value();
                            quote! {
                                #enum_name::#variant_name => serializer.serialize_bool(#value)
                            }
                        },
                        Lit::Int(i) => {
                            let value = i.base10_digits();
                            quote! {
                                #enum_name::#variant_name => serializer.serialize_i64(#value.parse::<i64>().unwrap())
                            }
                        },
                        Lit::Float(f) => {
                            let value = f.base10_digits();
                            quote! {
                                #enum_name::#variant_name => serializer.serialize_f64(#value.parse::<f64>().unwrap())
                            }
                        },
                        _ => quote! {
                            #enum_name::#variant_name => panic!("Unsupported literal type for variant")
                        },
                    }
                }).collect::<Vec<_>>();

                // Generate match arms for different visitor types
                let str_match_arms = variant_literals
                    .iter()
                    .filter_map(|(variant_name, literal)| {
                        if let Lit::Str(s) = literal {
                            let value = s.value();
                            Some(quote! {
                                if v == #value {
                                    return Ok(#enum_name::#variant_name);
                                }
                            })
                        } else {
                            None
                        }
                    })
                    .collect::<Vec<_>>();

                let bool_match_arms = variant_literals
                    .iter()
                    .filter_map(|(variant_name, literal)| {
                        if let Lit::Bool(b) = literal {
                            let value = b.value();
                            Some(quote! {
                                if v == #value {
                                    return Ok(#enum_name::#variant_name);
                                }
                            })
                        } else {
                            None
                        }
                    })
                    .collect::<Vec<_>>();

                let int_match_arms = variant_literals
                    .iter()
                    .filter_map(|(variant_name, literal)| {
                        if let Lit::Int(i) = literal {
                            let value = i.base10_digits();
                            Some(quote! {
                                if v.to_string() == #value {
                                    return Ok(#enum_name::#variant_name);
                                }
                            })
                        } else {
                            None
                        }
                    })
                    .collect::<Vec<_>>();

                let float_match_arms = variant_literals
                    .iter()
                    .filter_map(|(variant_name, literal)| {
                        if let Lit::Float(f) = literal {
                            let value = f.base10_digits();
                            Some(quote! {
                                if (v - #value.parse::<f64>().unwrap()).abs() < std::f64::EPSILON {
                                    return Ok(#enum_name::#variant_name);
                                }
                            })
                        } else {
                            None
                        }
                    })
                    .collect::<Vec<_>>();

                // Generate visitor functions for different types
                let str_visitor = quote! {
                    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
                    where
                        E: serde::de::Error,
                    {
                        #(#str_match_arms)*
                        Err(serde::de::Error::custom(format!("Unknown variant: {}", v)))
                    }
                };

                let bool_visitor = quote! {
                    fn visit_bool<E>(self, v: bool) -> Result<Self::Value, E>
                    where
                        E: serde::de::Error,
                    {
                        #(#bool_match_arms)*
                        Err(serde::de::Error::custom(format!("Unknown variant: {}", v)))
                    }
                };

                let int_visitor = quote! {
                    fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E>
                    where
                        E: serde::de::Error,
                    {
                        #(#int_match_arms)*
                        Err(serde::de::Error::custom(format!("Unknown variant: {}", v)))
                    }

                    fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
                    where
                        E: serde::de::Error,
                    {
                        #(#int_match_arms)*
                        Err(serde::de::Error::custom(format!("Unknown variant: {}", v)))
                    }
                };

                let float_visitor = quote! {
                    fn visit_f64<E>(self, v: f64) -> Result<Self::Value, E>
                    where
                        E: serde::de::Error,
                    {
                        #(#float_match_arms)*
                        Err(serde::de::Error::custom(format!("Unknown variant: {}", v)))
                    }
                };

                // Create combined visitor structure
                let visitor_ident =
                    syn::Ident::new(&format!("{}Visitor", enum_name), enum_name.span());
                let visitor_struct = quote! {
                    struct #visitor_ident;

                    impl<'de> serde::de::Visitor<'de> for #visitor_ident {
                        type Value = #enum_name;

                        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                            write!(formatter, "a literal matching one of the enum variants")
                        }

                        #str_visitor
                        #bool_visitor
                        #int_visitor
                        #float_visitor
                    }
                };

                // Generate final code with all implementations
                quote! {
                    #[derive(Clone, PartialEq, Eq, Debug)]
                    #item

                    impl serde::Serialize for #enum_name {
                        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
                        where
                            S: serde::Serializer,
                        {
                            match self {
                                #(#serialize_match_arms),*
                            }
                        }
                    }

                    impl<'de> serde::Deserialize<'de> for #enum_name {
                        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
                        where
                            D: serde::Deserializer<'de>,
                        {
                            deserializer.deserialize_any(#visitor_ident)
                        }
                    }

                    #visitor_struct
                }
            } else {
                // Regular enum handling
                quote! {
                    #[derive(Serialize, Deserialize)]
                    #[serde(untagged)]
                    #item
                }
            }
        }

        _ => panic!(
            "The #[literal] attribute can only be used with structs, enums and enum variants"
        ),
    };

    TokenStream::from(expanded)
}

struct LitDef<'a, L: ToTokens> {
    literal_type: proc_macro2::TokenStream,
    literal: L,
    trait_ident: proc_macro2::TokenStream,
    struct_ident: &'a syn::Ident,
}

fn struct_lit_trait_code<L>(def: LitDef<L>) -> proc_macro2::TokenStream
where
    L: ToTokens,
{
    let LitDef {
        literal_type,
        literal,
        trait_ident,
        struct_ident,
    } = def;

    quote! {
    impl litty::#trait_ident for #struct_ident {
        const LIT: #literal_type = #literal;
    }

    impl serde::Serialize for #struct_ident {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: serde::Serializer,
        {
            use litty::#trait_ident;
            Self::lit_serialize(serializer)
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
