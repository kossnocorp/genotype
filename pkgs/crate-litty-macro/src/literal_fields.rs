use proc_macro::TokenStream;
use quote::quote;
use syn::parse::Parser;
use syn::{
    Data, DeriveInput, Error, Fields, Ident, Lit, Meta, MetaList, MetaNameValue, Token,
    parse_macro_input, punctuated::Punctuated,
};

pub fn macro_derive_literals(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    match expand_literals(&input, LiteralsMode::Both) {
        Ok(tokens) => tokens.into(),
        Err(err) => err.to_compile_error().into(),
    }
}

pub fn macro_derive_serialize_literals(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    match expand_literals(&input, LiteralsMode::SerializeOnly) {
        Ok(tokens) => tokens.into(),
        Err(err) => err.to_compile_error().into(),
    }
}

pub fn macro_derive_deserialize_literals(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    match expand_literals(&input, LiteralsMode::DeserializeOnly) {
        Ok(tokens) => tokens.into(),
        Err(err) => err.to_compile_error().into(),
    }
}

enum LiteralsMode {
    Both,
    SerializeOnly,
    DeserializeOnly,
}

struct LiteralField {
    ident: Ident,
    ty: proc_macro2::TokenStream,
    value: proc_macro2::TokenStream,
    lit: Lit,
}

fn expand_literals(
    input: &DeriveInput,
    mode: LiteralsMode,
) -> Result<proc_macro2::TokenStream, Error> {
    if matches!(input.data, Data::Enum(_)) {
        return expand_literal_enum(input, mode);
    }

    let struct_ident = &input.ident;

    if input.generics.lifetimes().next().is_some() {
        return Err(Error::new_spanned(
            &input.generics,
            "Literals does not support lifetime generics",
        ));
    }

    let literal_fields = parse_literal_fields(&input.attrs)?;

    let fields = match &input.data {
        syn::Data::Struct(data) => match &data.fields {
            Fields::Named(named) => named.named.iter().cloned().collect::<Vec<_>>(),
            _ => {
                return Err(Error::new_spanned(
                    &data.fields,
                    "Literals only supports structs with named fields",
                ));
            }
        },
        _ => {
            return Err(Error::new_spanned(
                &input.ident,
                "Literals can only be used with structs",
            ));
        }
    };

    if literal_fields.is_empty() {
        return Err(Error::new_spanned(
            &input.ident,
            "Literals requires a #[literals(...)] attribute",
        ));
    }

    let mut field_names = std::collections::BTreeSet::new();
    for field in &fields {
        if let Some(ident) = &field.ident {
            field_names.insert(ident.to_string());
        }
    }

    for lit_field in &literal_fields {
        let name = lit_field.ident.to_string();
        if field_names.contains(&name) {
            return Err(Error::new_spanned(
                &lit_field.ident,
                "Literal field conflicts with struct field name",
            ));
        }
    }

    let serialize_ident = Ident::new(
        &format!("__{}LiteralsSerialize", struct_ident),
        struct_ident.span(),
    );
    let deserialize_ident = Ident::new(
        &format!("__{}LiteralsDeserialize", struct_ident),
        struct_ident.span(),
    );

    let literal_field_names: Vec<_> = literal_fields.iter().map(|field| &field.ident).collect();
    let literal_field_types: Vec<_> = literal_fields.iter().map(|field| &field.ty).collect();
    let literal_field_values: Vec<_> = literal_fields.iter().map(|field| &field.value).collect();

    let serialize_fields = fields.iter().map(|field| {
        let attrs = &field.attrs;
        let ident = field.ident.as_ref().expect("Named field");
        let ty = &field.ty;
        quote! { #(#attrs)* #ident: &'__litty #ty }
    });

    let deserialize_fields = fields.iter().map(|field| {
        let attrs = &field.attrs;
        let ident = field.ident.as_ref().expect("Named field");
        let ty = &field.ty;
        quote! { #(#attrs)* #ident: #ty }
    });

    let serialize_init_fields = fields.iter().map(|field| {
        let ident = field.ident.as_ref().expect("Named field");
        quote! { #ident: &self.#ident }
    });

    let deserialize_init_fields = fields.iter().map(|field| {
        let ident = field.ident.as_ref().expect("Named field");
        quote! { #ident: helper.#ident }
    });

    let mut helper_generics = input.generics.clone();
    helper_generics
        .params
        .insert(0, syn::parse_quote!('__litty));
    let (helper_impl_generics, _helper_ty_generics, helper_where_clause) =
        helper_generics.split_for_impl();

    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    let serialize_where_clause = add_serialize_bounds(&input.generics);
    let deserialize_generics = add_deserialize_lifetime(&input.generics);
    let (deserialize_impl_generics, _deserialize_ty_generics, deserialize_where_clause) =
        deserialize_generics.split_for_impl();

    let literal_checks = literal_fields.iter().map(|field| {
        let ident = &field.ident;
        let value = &field.value;
        let expected = &field.lit;
        quote! {
            if helper.#ident != #value {
                return Err(serde::de::Error::custom(format!(
                    "expected {} = {:?}, got {:?}",
                    stringify!(#ident),
                    #expected,
                    helper.#ident
                )));
            }
        }
    });

    let serialize_tokens = quote! {
        #[derive(serde::Serialize)]
        struct #serialize_ident #helper_impl_generics #helper_where_clause {
            #(#serialize_fields,)*
            #(#literal_field_names: #literal_field_types,)*
        }

        impl #impl_generics serde::Serialize for #struct_ident #ty_generics #serialize_where_clause {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                let helper = #serialize_ident {
                    #(#serialize_init_fields,)*
                    #(#literal_field_names: #literal_field_values,)*
                };
                helper.serialize(serializer)
            }
        }
    };

    let deserialize_tokens = quote! {
        #[derive(serde::Deserialize)]
        struct #deserialize_ident #impl_generics #where_clause {
            #(#deserialize_fields,)*
            #(#literal_field_names: #literal_field_types,)*
        }

        impl #deserialize_impl_generics serde::Deserialize<'__de> for #struct_ident #ty_generics #deserialize_where_clause {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: serde::Deserializer<'__de>,
            {
                let helper = #deserialize_ident::deserialize(deserializer)?;
                #(#literal_checks)*
                Ok(Self {
                    #(#deserialize_init_fields,)*
                })
            }
        }
    };

    Ok(match mode {
        LiteralsMode::Both => quote! {
            #serialize_tokens
            #deserialize_tokens
        },
        LiteralsMode::SerializeOnly => serialize_tokens,
        LiteralsMode::DeserializeOnly => deserialize_tokens,
    })
}

struct EnumLiteralVariant {
    ident: Ident,
    lit: Lit,
}

fn expand_literal_enum(
    input: &DeriveInput,
    mode: LiteralsMode,
) -> Result<proc_macro2::TokenStream, Error> {
    let enum_ident = &input.ident;

    if input.generics.lifetimes().next().is_some() {
        return Err(Error::new_spanned(
            &input.generics,
            "Literals does not support lifetime generics for enums",
        ));
    }

    let Data::Enum(data_enum) = &input.data else {
        unreachable!("expand_literal_enum is called only for enums")
    };

    let mut variants: Vec<EnumLiteralVariant> = vec![];
    for variant in &data_enum.variants {
        let mut literal: Option<Lit> = None;

        for attr in &variant.attrs {
            if let syn::Meta::List(syn::MetaList { path, tokens, .. }) = &attr.meta {
                let is_literal = path
                    .segments
                    .iter()
                    .last()
                    .is_some_and(|segment| segment.ident == "literal");

                if is_literal {
                    if !matches!(variant.fields, Fields::Unit) {
                        return Err(Error::new_spanned(
                            &variant.ident,
                            "SerializeLiterals/DeserializeLiterals only support unit enum variants with #[literal(...)]",
                        ));
                    }

                    literal = Some(syn::parse2::<Lit>(tokens.clone()).map_err(|_| {
                        Error::new_spanned(
                            tokens,
                            "literal enum variant value must be a string, bool, int, or float literal",
                        )
                    })?);
                }
            }
        }

        if let Some(lit) = literal {
            variants.push(EnumLiteralVariant {
                ident: variant.ident.clone(),
                lit,
            });
        }
    }

    if variants.is_empty() {
        return Err(Error::new_spanned(
            enum_ident,
            "SerializeLiterals/DeserializeLiterals enum requires at least one #[literal(...)] unit variant",
        ));
    }

    let (impl_generics, ty_generics, _where_clause) = input.generics.split_for_impl();
    let serialize_where_clause = add_serialize_bounds(&input.generics);
    let deserialize_generics = add_deserialize_lifetime(&input.generics);
    let (deserialize_impl_generics, _deserialize_ty_generics, deserialize_where_clause) =
        deserialize_generics.split_for_impl();

    let mut helper_structs = quote! {};
    let mut serialize_arms = quote! {};
    let mut deserialize_arms = quote! {};

    let helper_suffix = match mode {
        LiteralsMode::Both => "Serde",
        LiteralsMode::SerializeOnly => "Serialize",
        LiteralsMode::DeserializeOnly => "Deserialize",
    };

    let literal_value_ident = Ident::new(
        &format!("__{}LittyLiteralValue{}", enum_ident, helper_suffix),
        enum_ident.span(),
    );
    let literal_visitor_ident = Ident::new(
        &format!("__{}LittyLiteralVisitor{}", enum_ident, helper_suffix),
        enum_ident.span(),
    );

    for variant in &variants {
        let variant_ident = &variant.ident;
        let helper_ident = Ident::new(
            &format!(
                "__{}{}LittyLiteral{}",
                enum_ident, variant_ident, helper_suffix
            ),
            variant_ident.span(),
        );

        let (trait_ident, literal_type, literal_value) = literal_trait_def(&variant.lit)?;

        helper_structs.extend(quote! {
            struct #helper_ident;
            impl litty::#trait_ident for #helper_ident {
                const LIT: #literal_type = #literal_value;
            }
        });

        serialize_arms.extend(quote! {
            #enum_ident::#variant_ident => <#helper_ident as litty::#trait_ident>::lit_serialize(serializer),
        });

        let deserialize_match = match &variant.lit {
            Lit::Str(_) => quote! {
                #literal_value_ident::Str(value) if value == <#helper_ident as litty::#trait_ident>::LIT => {
                    return Ok(#enum_ident::#variant_ident);
                }
            },
            Lit::Bool(_) => quote! {
                #literal_value_ident::Bool(value) if value == <#helper_ident as litty::#trait_ident>::LIT => {
                    return Ok(#enum_ident::#variant_ident);
                }
            },
            Lit::Int(_) => quote! {
                #literal_value_ident::Int(value) if value == <#helper_ident as litty::#trait_ident>::LIT => {
                    return Ok(#enum_ident::#variant_ident);
                }
            },
            Lit::Float(_) => quote! {
                #literal_value_ident::Float(value) if value == <#helper_ident as litty::#trait_ident>::LIT => {
                    return Ok(#enum_ident::#variant_ident);
                }
            },
            _ => unreachable!(),
        };
        deserialize_arms.extend(deserialize_match);
    }

    let serialize_impl = quote! {
        impl #impl_generics serde::Serialize for #enum_ident #ty_generics #serialize_where_clause {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                match self {
                    #serialize_arms
                    _ => Err(serde::ser::Error::custom(
                        "SerializeLiterals only supports enum variants annotated with #[literal(...)]",
                    )),
                }
            }
        }
    };

    let deserialize_impl = quote! {
        enum #literal_value_ident {
            Str(String),
            Bool(bool),
            Int(i64),
            Float(f64),
        }

        struct #literal_visitor_ident;

        impl<'de> serde::de::Visitor<'de> for #literal_visitor_ident {
            type Value = #literal_value_ident;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a string, bool, integer, or float literal")
            }

            fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(#literal_value_ident::Str(value.to_owned()))
            }

            fn visit_string<E>(self, value: String) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(#literal_value_ident::Str(value))
            }

            fn visit_bool<E>(self, value: bool) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(#literal_value_ident::Bool(value))
            }

            fn visit_i64<E>(self, value: i64) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(#literal_value_ident::Int(value))
            }

            fn visit_u64<E>(self, value: u64) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                let value = i64::try_from(value)
                    .map_err(|_| E::custom("integer literal is out of i64 range"))?;
                Ok(#literal_value_ident::Int(value))
            }

            fn visit_f64<E>(self, value: f64) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(#literal_value_ident::Float(value))
            }
        }

        impl #deserialize_impl_generics serde::Deserialize<'__de> for #enum_ident #ty_generics #deserialize_where_clause {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: serde::Deserializer<'__de>,
            {
                let value = deserializer.deserialize_any(#literal_visitor_ident)?;

                match value {
                    #deserialize_arms
                    _ => Err(serde::de::Error::custom(
                        "unknown literal enum variant",
                    )),
                }
            }
        }
    };

    Ok(match mode {
        LiteralsMode::Both => quote! {
            #helper_structs
            #serialize_impl
            #deserialize_impl
        },
        LiteralsMode::SerializeOnly => quote! {
            #helper_structs
            #serialize_impl
        },
        LiteralsMode::DeserializeOnly => quote! {
            #helper_structs
            #deserialize_impl
        },
    })
}

fn literal_trait_def(
    lit: &Lit,
) -> Result<
    (
        proc_macro2::TokenStream,
        proc_macro2::TokenStream,
        proc_macro2::TokenStream,
    ),
    Error,
> {
    match lit {
        Lit::Str(lit_str) => Ok((
            quote! { LitStr },
            quote! { &'static str },
            quote! { #lit_str },
        )),
        Lit::Bool(lit_bool) => Ok((quote! { LitBool }, quote! { bool }, quote! { #lit_bool })),
        Lit::Int(lit_int) => {
            let value = lit_int
                .base10_digits()
                .parse::<i64>()
                .map_err(|_| Error::new_spanned(lit_int, "Invalid i64 literal"))?;
            Ok((quote! { LitInt }, quote! { i64 }, quote! { #value }))
        }
        Lit::Float(lit_float) => {
            let value = lit_float
                .base10_digits()
                .parse::<f64>()
                .map_err(|_| Error::new_spanned(lit_float, "Invalid f64 literal"))?;
            Ok((quote! { LitFloat }, quote! { f64 }, quote! { #value }))
        }
        _ => Err(Error::new_spanned(
            lit,
            "Literal values only support string, bool, int, or float literals",
        )),
    }
}

fn parse_literal_fields(attrs: &[syn::Attribute]) -> Result<Vec<LiteralField>, Error> {
    let mut literal_fields = Vec::new();

    for attr in attrs {
        let Meta::List(MetaList { path, tokens, .. }) = &attr.meta else {
            continue;
        };
        if !path.is_ident("literals") {
            continue;
        }

        let parser = Punctuated::<MetaNameValue, Token![,]>::parse_terminated;
        let items = parser.parse2(tokens.clone())?;
        for item in items {
            let ident = match item.path.get_ident() {
                Some(ident) => ident.clone(),
                None => {
                    return Err(Error::new_spanned(
                        item.path,
                        "Literal field name must be an identifier",
                    ));
                }
            };
            let lit = match item.value {
                syn::Expr::Lit(expr_lit) => expr_lit.lit,
                _ => {
                    return Err(Error::new_spanned(
                        item.value,
                        "Literal field value must be a literal",
                    ));
                }
            };

            let (ty, value) = literal_type_and_value(&lit)?;

            literal_fields.push(LiteralField {
                ident,
                ty,
                value,
                lit,
            });
        }
    }

    Ok(literal_fields)
}

fn literal_type_and_value(
    lit: &Lit,
) -> Result<(proc_macro2::TokenStream, proc_macro2::TokenStream), Error> {
    match lit {
        Lit::Str(lit_str) => Ok((quote! { &'static str }, quote! { #lit_str })),
        Lit::Bool(lit_bool) => Ok((quote! { bool }, quote! { #lit_bool })),
        Lit::Int(lit_int) => {
            let value = lit_int
                .base10_digits()
                .parse::<i64>()
                .map_err(|_| Error::new_spanned(lit_int, "Invalid i64 literal"))?;
            Ok((quote! { i64 }, quote! { #value }))
        }
        Lit::Float(lit_float) => {
            let value = lit_float
                .base10_digits()
                .parse::<f64>()
                .map_err(|_| Error::new_spanned(lit_float, "Invalid f64 literal"))?;
            Ok((quote! { f64 }, quote! { #value }))
        }
        _ => Err(Error::new_spanned(
            lit,
            "Literals only supports string, bool, int, or float literals",
        )),
    }
}

fn add_serialize_bounds(generics: &syn::Generics) -> Option<syn::WhereClause> {
    let mut where_clause = generics
        .where_clause
        .clone()
        .unwrap_or_else(|| syn::WhereClause {
            where_token: Default::default(),
            predicates: Punctuated::new(),
        });

    for param in &generics.params {
        if let syn::GenericParam::Type(ty) = param {
            let ident = &ty.ident;
            where_clause
                .predicates
                .push(syn::parse_quote!(#ident: serde::Serialize));
        }
    }

    if where_clause.predicates.is_empty() {
        None
    } else {
        Some(where_clause)
    }
}

fn add_deserialize_lifetime(generics: &syn::Generics) -> syn::Generics {
    let mut generics = generics.clone();
    generics.params.insert(0, syn::parse_quote!('__de));

    let mut where_clause = generics
        .where_clause
        .take()
        .unwrap_or_else(|| syn::WhereClause {
            where_token: Default::default(),
            predicates: Punctuated::new(),
        });

    for param in &generics.params {
        if let syn::GenericParam::Type(ty) = param {
            let ident = &ty.ident;
            where_clause
                .predicates
                .push(syn::parse_quote!(#ident: serde::Deserialize<'__de>));
        }
    }

    if where_clause.predicates.is_empty() {
        generics.where_clause = None;
    } else {
        generics.where_clause = Some(where_clause);
    }
    generics
}
