use proc_macro::TokenStream;
use quote::{quote, ToTokens};
use syn::{parse_macro_input, Ident, Lit};

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
                lit_trait_code(&item.ident, &parse_macro_input!(attr as Lit))
            };

            quote! {
                #[derive(Clone, Default, Eq, PartialEq)]
                #item

                #traits_code
            }
        }

        syn::Item::Enum(mut item) => {
            let enum_ident = &item.ident;

            // These simultaneously indicate if Debug and Hash traits are needed and store
            // the variants for them. The trait variants are being collected while processing
            // the enum variants.
            let mut debug_variants: Option<proc_macro2::TokenStream> = None;
            let mut hash_variants: Option<proc_macro2::TokenStream> = None;

            // Find what extra traits we need to implement (Debug, Hash).
            for attr in &mut item.attrs {
                match &mut attr.meta {
                    syn::Meta::List(syn::MetaList { path, tokens, .. }) => {
                        if path.is_ident("derive".into()) {
                            // Remove and register Debug and Hash traits
                            *tokens = {
                                let mut iter = tokens.clone().into_iter().peekable();
                                let mut traits_tokens = proc_macro2::TokenStream::new();
                                while let Some(token) = iter.next() {
                                    if let proc_macro2::TokenTree::Ident(ident) = &token {
                                        if ident == "Debug" {
                                            debug_variants = Some(quote! {});
                                            // Skip following comma, if any
                                            if let Some(proc_macro2::TokenTree::Punct(p)) =
                                                iter.peek()
                                            {
                                                if p.as_char() == ',' {
                                                    iter.next();
                                                }
                                            }
                                            continue;
                                        } else if ident == "Hash" {
                                            hash_variants = Some(quote! {});
                                            // Skip following comma, if any
                                            if let Some(proc_macro2::TokenTree::Punct(punct)) =
                                                iter.peek()
                                            {
                                                if punct.as_char() == ',' {
                                                    iter.next();
                                                }
                                            }
                                            continue;
                                        }
                                    }
                                    traits_tokens.extend(std::iter::once(token));
                                }
                                traits_tokens
                            };
                        }
                    }
                    _ => {}
                }
            }

            let mut traits_code = quote! {};

            // Iterate each variant
            for variant in &mut item.variants {
                let variant_ident = &variant.ident;
                let variant_name = variant_ident.to_string();
                let mut lit: Option<Lit> = None;

                // Process attributes
                variant.attrs.retain(|attr| {
                    // Extract and remove literal attribute
                    if let syn::Meta::List(syn::MetaList { path, tokens, .. }) = &attr.meta {
                        let is_literal = path
                            .segments
                            .iter()
                            .last()
                            .map_or(false, |s| s.ident == "literal");
                        if is_literal {
                            lit = if let Ok(lit) = syn::parse2::<Lit>(tokens.clone()) {
                                Some(lit)
                            } else {
                                None
                            };

                            return false;
                        }
                    }

                    // Keep other attributes
                    true
                });

                let variant_traits_code = if let Some(lit) = lit {
                    let lit_name = enum_ident.to_string() + variant_ident.to_string().as_str();
                    let lit_ident = Ident::new(&lit_name, variant.ident.span());
                    let trait_ident = lit_trait_ident(&lit);

                    // Generate the literal struct
                    let lit_struct = quote! {
                        #[derive(Clone, Default, Eq, PartialEq)]
                        struct #lit_ident;
                    };

                    // Generate the literal traits
                    let literal_traits = lit_trait_code(&lit_ident, &lit);

                    // Add serde attributes
                    let ser_str = format!("<{lit_name} as litty::{trait_ident}>::lit_serialize");
                    let de_str = format!("<{lit_name} as litty::{trait_ident}>::lit_deserialize");
                    variant.attrs.push(syn::parse_quote! {
                        #[serde(
                            serialize_with = #ser_str,
                            deserialize_with = #de_str
                        )]
                    });

                    // Register the literal debug variant
                    if let Some(variants) = &mut debug_variants {
                        variants.extend(quote! {
                            #enum_ident::#variant_ident => { #lit_ident.fmt(f) },
                        });
                    }

                    // Register the literal hash variant
                    if let Some(variants) = &mut hash_variants {
                        variants.extend(quote! {
                            #enum_ident::#variant_ident => { #lit_ident.hash(state) },
                        });
                    }

                    quote! {
                        #lit_struct

                        #literal_traits
                    }
                } else {
                    match &variant.fields {
                        // Unit variant (e.g. Variant)
                        syn::Fields::Unit => {
                            // Register the unit variant debug variant
                            if let Some(variants) = &mut debug_variants {
                                variants.extend(quote! {
                                    #enum_ident::#variant_ident => { write!(f, #variant_name) },
                                });
                            }

                            // Register the unit variant hash variant
                            if let Some(variants) = &mut hash_variants {
                                variants.extend(quote! {
                                    #enum_ident::#variant_ident => {},
                                });
                            }
                        }

                        // Named variant (e.g. Variant { str: String })
                        syn::Fields::Named(fields) => {
                            let field_idents = fields
                                .named
                                .iter()
                                .map(|field| {
                                    field
                                        .clone()
                                        .ident
                                        .expect("Named field should have an identifier")
                                })
                                .collect::<Vec<_>>();

                            let mut fields_code = quote! {};
                            for ident in &field_idents {
                                fields_code.extend(quote! { #ident, });
                            }

                            // Register the named variant debug variant
                            if let Some(variants) = &mut debug_variants {
                                let write_str = format!(
                                    "{variant_name} {{{{ {} }}}}",
                                    field_idents
                                        .iter()
                                        .map(|ident| { format!("{ident}: {}", "{:?}") })
                                        .collect::<Vec<_>>()
                                        .join(", ")
                                );

                                variants.extend(quote! {
                                    #enum_ident::#variant_ident { #fields_code } => { write!(f, #write_str,  #fields_code) },
                                });
                            }

                            // Register the named variant hash variant
                            if let Some(variants) = &mut hash_variants {
                                let mut fields_hash_code = quote! {};
                                for ident in &field_idents {
                                    fields_hash_code.extend(quote! {
                                        #ident.hash(state);
                                    });
                                }

                                variants.extend(quote! {
                                    #enum_ident::#variant_ident { #fields_code } => {
                                        #fields_hash_code
                                    },
                                });
                            }
                        }

                        // Tuple variant (e.g. Variant(String) })
                        syn::Fields::Unnamed(fields) => {
                            let field_idents = fields
                                .unnamed
                                .iter()
                                .enumerate()
                                .map(|(i, _)| {
                                    Ident::new(&format!("field_{}", i), variant.ident.span())
                                })
                                .collect::<Vec<_>>();

                            let mut fields_code = quote! {};
                            for field in &field_idents {
                                fields_code.extend(quote! { #field, });
                            }

                            // Register the named variant debug variant
                            if let Some(variants) = &mut debug_variants {
                                let write_str = format!(
                                    "{variant_name}({})",
                                    field_idents
                                        .iter()
                                        .map(|_| { "{:?}" })
                                        .collect::<Vec<_>>()
                                        .join(", ")
                                );

                                variants.extend(quote! {
                                    #enum_ident::#variant_ident(#fields_code) => { write!(f, #write_str, #fields_code) },
                                });
                            }

                            // Register the named variant hash variant
                            if let Some(variants) = &mut hash_variants {
                                let mut fields_hash_code = quote! {};
                                for field_ident in &field_idents {
                                    fields_hash_code.extend(quote! {
                                        #field_ident.hash(state);
                                    });
                                }

                                variants.extend(quote! {
                                    #enum_ident::#variant_ident(#fields_code) => {
                                        #fields_hash_code
                                    },
                                });
                            }
                        }
                    }

                    quote! {}
                };

                traits_code.extend(variant_traits_code);
            }

            // Implement Debug
            let debug_code = if let Some(variants) = debug_variants {
                quote! {
                    impl std::fmt::Debug for #enum_ident {
                        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                            match self {
                                #variants
                            }
                        }
                    }
                }
            } else {
                quote! {}
            };

            // Implement Hash
            let hash_code = if let Some(variants) = hash_variants {
                quote! {
                    impl std::hash::Hash for #enum_ident {
                        fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
                            match self {
                                #variants
                            }
                        }
                    }
                }
            } else {
                quote! {}
            };

            quote! {
                #[derive(Serialize, Deserialize)]
                #[serde(untagged)]
                #item

                #hash_code

                #debug_code

                #traits_code
            }
        }

        _ => panic!(
            "The #[literal] attribute can only be used with structs, enums and enum variants"
        ),
    };

    TokenStream::from(expanded)
}

fn lit_trait_code(struct_ident: &Ident, lit: &Lit) -> proc_macro2::TokenStream {
    let trait_ident = lit_trait_ident(lit);
    match lit {
        Lit::Str(lit_str) => struct_lit_trait_code(LitDef {
            literal_type: quote! { &'static str },
            literal: lit_str.value(),
            trait_ident,
            struct_ident,
        }),

        Lit::Bool(lit_bool) => struct_lit_trait_code(LitDef {
            literal_type: quote! { bool },
            literal: lit_bool.value(),
            trait_ident,
            struct_ident,
        }),

        Lit::Int(lit_int) => struct_lit_trait_code(LitDef {
            literal_type: quote! { i64 },
            literal: lit_int
                .base10_digits()
                .parse::<i64>()
                .expect("Invalid i64 literal"),
            trait_ident,
            struct_ident,
        }),

        Lit::Float(lit_float) => struct_lit_trait_code(LitDef {
            literal_type: quote! { f64 },
            literal: lit_float
                .base10_digits()
                .parse::<f64>()
                .expect("Invalid f64 literal"),
            trait_ident,
            struct_ident,
        }),

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
