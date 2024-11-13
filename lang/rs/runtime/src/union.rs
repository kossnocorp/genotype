use proc_macro::TokenStream;
use quote::{quote, ToTokens};
use syn::{parse_macro_input, DeriveInput, Item, ItemEnum, Lit, Meta};

pub fn macro_attribute(_attr: TokenStream, input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let item: Item = input.into();

    let expanded = match item {
        syn::Item::Enum(item) => {
            // [TODO]
            let serde_code = enum_serde_code(&item);

            quote! {
                #item

                #serde_code
            }
        }

        _ => panic!("The #[enum] attribute can only be used with enums"),
    };

    TokenStream::from(expanded)
}

fn enum_serde_code(item: &ItemEnum) -> proc_macro2::TokenStream {
    let enum_name = &item.ident;
    let variants = &item.variants;

    let mut serialize_arms: Vec<proc_macro2::TokenStream> = Vec::new();
    let mut deserialize_arms: Vec<proc_macro2::TokenStream> = Vec::new();
    let mut variant_names: Vec<proc_macro2::TokenStream> = Vec::new();

    for variant in variants {
        let variant_ident = &variant.ident;

        // let mut literal_value = None;
        for attr in &variant.attrs {
            if attr.path().is_ident("union_literal") {
                let _ = attr.parse_nested_meta(|meta| {
                    // meta.
                    // Meta::NameValue(meta) => {
                    //     if let Lit::Str(lit_str) = meta.lit {
                    //         literal_value = Some(lit_str.value());
                    //     }
                    // }
                    // _ => panic!("Only string literals are supported"),
                    Ok(())
                });
                // if let Ok(Meta::NameValue(meta)) = attr.parse_meta() {
                //     if let Lit::Str(lit_str) = meta.lit {
                //         literal_value = Some(lit_str.value());
                //     }
                // }
            }
        }

        // match &variant.fields {
        //     syn::Fields::Unit => {
        //         // Use the literal value if provided, otherwise use the variant name in lowercase
        //         let variant_name =
        //             literal_value.unwrap_or_else(|| variant_ident.to_string().to_lowercase());
        //         variant_names.push(variant_name.clone());

        //         serialize_arms.push(quote! {
        //             #enum_name::#variant_ident => serializer.serialize_str(#variant_name),
        //         });
        //         deserialize_arms.push(quote! {
        //             #variant_name => Ok(#enum_name::#variant_ident),
        //         });
        //     }
        //     syn::Fields::Unnamed(fields) => {
        //         if fields.unnamed.len() == 1 {
        //             let field_type = &fields.unnamed.first().unwrap().ty;

        //             // Delegate serialization/deserialization to the inner type
        //             serialize_arms.push(quote! {
        //                 #enum_name::#variant_ident(inner) => inner.serialize(serializer),
        //             });
        //             deserialize_arms.push(quote! {
        //                 value => {
        //                     let inner = <#field_type>::deserialize(value.into_deserializer())?;
        //                     Ok(#enum_name::#variant_ident(inner))
        //                 },
        //             });
        //         } else {
        //             panic!("Only single-field tuple variants are supported");
        //         }
        //     }
        //     _ => panic!("Named fields are not supported"),
        // }
    }

    let variant_names_array = variant_names
        .iter()
        .map(|name| quote! { #name })
        .collect::<Vec<_>>();

    quote! {
        // impl serde::Serialize for #enum_name {
        //     fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        //     where S: serde::Serializer {
        //         // match self {
        //         //     #(#serialize_arms)*
        //         // }
        //         Err("Nope".into())
        //     }
        // }

        // impl<'de> serde::Deserialize<'de> for #enum_name {
        //     fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        //     where D: serde::Deserializer<'de> {
        // //         let value = String::deserialize(deserializer)?;
        // //         match value.as_str() {
        // //             #(#deserialize_arms)*

        // //             _ => Err(serde::de::Error::unknown_variant(&value, &[#(#variant_names_array),*])),
        // //         }
        //         Err(serde::de::Error::unknown_variant)
        //     }
        // }
    }
}
