use heck::ToSnakeCase;
use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{Data, DeriveInput, Fields, ItemStruct, parse_macro_input};

#[proc_macro_derive(Visitor, attributes(visit))]
pub fn derive_visitor(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    if !input.generics.params.is_empty() || input.generics.where_clause.is_some() {
        return syn::Error::new_spanned(
            &input.ident,
            "Visitor derive does not support generic types",
        )
        .to_compile_error()
        .into();
    }

    let type_name = input.ident.to_string();
    let Some(prefix) = type_prefix(&type_name) else {
        return syn::Error::new_spanned(
            &input.ident,
            "Visitor derive expects a prefixed type name (e.g. GtAlias, PyModule)",
        )
        .to_compile_error()
        .into();
    };

    let visitor_trait_ident = format_ident!("{}Visitor", prefix);
    let visit_method_ident = format_ident!("visit_{}", type_method_suffix(&type_name));
    let ident = input.ident.clone();

    let (children, is_enum) = match &input.data {
        Data::Struct(data_struct) => (struct_children(&data_struct.fields), false),
        Data::Enum(data_enum) => (
            data_enum
                .variants
                .iter()
                .map(|variant| {
                    let variant_ident = &variant.ident;

                    match &variant.fields {
                        Fields::Named(fields) => {
                            let names = fields
                                .named
                                .iter()
                                .filter_map(|field| {
                                    if has_visit_attr(field) {
                                        field.ident.clone()
                                    } else {
                                        None
                                    }
                                })
                                .collect::<Vec<_>>();

                            let all_names = fields
                                .named
                                .iter()
                                .map(|field| field.ident.clone().expect("named field"))
                                .collect::<Vec<_>>();

                            quote! {
                                Self::#variant_ident { #(#all_names),* } => {
                                    #(crate::visitor::Traverse::traverse(#names, visitor);)*
                                }
                            }
                        }
                        Fields::Unnamed(fields) => {
                            let names = fields
                                .unnamed
                                .iter()
                                .enumerate()
                                .map(|(index, _)| format_ident!("field_{index}"))
                                .collect::<Vec<_>>();

                            let traversable = fields
                                .unnamed
                                .iter()
                                .enumerate()
                                .filter_map(|(index, field)| {
                                    if has_visit_attr(field) {
                                        Some(format_ident!("field_{index}"))
                                    } else {
                                        None
                                    }
                                })
                                .collect::<Vec<_>>();

                            quote! {
                                Self::#variant_ident(#(#names),*) => {
                                    #(crate::visitor::Traverse::traverse(#traversable, visitor);)*
                                }
                            }
                        }
                        Fields::Unit => {
                            quote! { Self::#variant_ident => {} }
                        }
                    }
                })
                .collect::<Vec<_>>(),
            true,
        ),
        _ => {
            return syn::Error::new_spanned(&ident, "Visitor derive supports structs and enums")
                .to_compile_error()
                .into();
        }
    };

    let impl_tokens = if is_enum {
        quote! {
            impl<V> crate::visitor::Traverse<V> for #ident
            where
                V: crate::visitor::#visitor_trait_ident + ?Sized,
            {
                fn traverse(&mut self, visitor: &mut V) {
                    visitor.#visit_method_ident(self);
                    match self {
                        #(#children),*
                    }
                }
            }
        }
    } else {
        quote! {
            impl<V> crate::visitor::Traverse<V> for #ident
            where
                V: crate::visitor::#visitor_trait_ident + ?Sized,
            {
                fn traverse(&mut self, visitor: &mut V) {
                    visitor.#visit_method_ident(self);
                    #(#children)*
                }
            }
        }
    };

    impl_tokens.into()
}

#[proc_macro_attribute]
pub fn visitor(args: TokenStream, input: TokenStream) -> TokenStream {
    let nodes = parse_macro_input!(args with syn::punctuated::Punctuated::<syn::Type, syn::Token![,]>::parse_terminated);
    let item = parse_macro_input!(input as ItemStruct);

    if !item.fields.is_empty() {
        return syn::Error::new_spanned(&item.ident, "visitor marker must be a unit struct")
            .to_compile_error()
            .into();
    }

    let vis = item.vis;
    let trait_ident = item.ident;

    let methods = nodes
        .iter()
        .map(|node| {
            let type_ident = match node {
                syn::Type::Path(path) => path
                    .path
                    .segments
                    .last()
                    .map(|segment| segment.ident.clone()),
                _ => None,
            };

            let Some(type_ident) = type_ident else {
                return syn::Error::new_spanned(node, "visitor node must be a named type")
                    .to_compile_error();
            };

            let method_ident =
                format_ident!("visit_{}", type_method_suffix(&type_ident.to_string()));
            quote! {
                fn #method_ident(&mut self, _node: &mut #node) {}
            }
        })
        .collect::<Vec<_>>();

    quote! {
        #vis trait #trait_ident {
            #(#methods)*
        }
    }
    .into()
}

fn struct_children(fields: &Fields) -> Vec<proc_macro2::TokenStream> {
    match fields {
        Fields::Named(fields) => fields
            .named
            .iter()
            .filter_map(|field| {
                if !has_visit_attr(field) {
                    return None;
                }

                let field_ident = field.ident.clone().expect("named field");
                Some(quote! {
                    crate::visitor::Traverse::traverse(&mut self.#field_ident, visitor);
                })
            })
            .collect(),
        Fields::Unnamed(fields) => fields
            .unnamed
            .iter()
            .enumerate()
            .filter_map(|(index, field)| {
                if !has_visit_attr(field) {
                    return None;
                }

                let index = syn::Index::from(index);
                Some(quote! {
                    crate::visitor::Traverse::traverse(&mut self.#index, visitor);
                })
            })
            .collect(),
        Fields::Unit => Vec::new(),
    }
}

fn has_visit_attr(field: &syn::Field) -> bool {
    field.attrs.iter().any(|attr| attr.path().is_ident("visit"))
}

fn type_method_suffix(type_name: &str) -> String {
    let stripped = if let Some(prefix) = type_prefix(type_name) {
        type_name.strip_prefix(&prefix).unwrap_or(type_name)
    } else {
        type_name
    };

    if stripped == "DependencyIdent" {
        return "dependency".into();
    }

    stripped.to_snake_case()
}

fn type_prefix(type_name: &str) -> Option<String> {
    let chars = type_name.chars().collect::<Vec<_>>();
    if chars.len() < 3 {
        return None;
    }

    for index in 1..(chars.len() - 1) {
        if chars[index].is_uppercase() && chars[index + 1].is_lowercase() {
            let prefix = chars[..index].iter().collect::<String>();
            if !prefix.is_empty() {
                return Some(prefix);
            }
        }
    }

    None
}
