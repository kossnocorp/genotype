use heck::ToSnakeCase;
use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{
    Data, DeriveInput, Fields, Ident, ItemStruct, Token,
    parse::{Parse, ParseStream},
    parse_macro_input,
    punctuated::Punctuated,
};

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
    let visitor_mut_trait_ident = format_ident!("{}VisitorMut", prefix);
    let visit_method_ident = format_ident!("visit_{}", type_method_suffix(&type_name));
    let visit_method_mut_ident = format_ident!("visit_{}_mut", type_method_suffix(&type_name));
    let ident = input.ident.clone();

    let (children, children_mut, is_enum) = match &input.data {
        Data::Struct(data_struct) => (
            struct_children(&data_struct.fields, TraversalMode::Immutable),
            struct_children(&data_struct.fields, TraversalMode::Mutable),
            false,
        ),
        Data::Enum(data_enum) => (
            enum_children(data_enum, TraversalMode::Immutable),
            enum_children(data_enum, TraversalMode::Mutable),
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
                fn traverse(&self, visitor: &mut V) {
                    visitor.#visit_method_ident(self);
                    match self {
                        #(#children),*
                    }
                }
            }

            impl<V> crate::visitor::TraverseMut<V> for #ident
            where
                V: crate::visitor::#visitor_mut_trait_ident + ?Sized,
            {
                fn traverse_mut(&mut self, visitor: &mut V) {
                    visitor.#visit_method_ident(self);
                    visitor.#visit_method_mut_ident(self);
                    match self {
                        #(#children_mut),*
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
                fn traverse(&self, visitor: &mut V) {
                    visitor.#visit_method_ident(self);
                    #(#children)*
                }
            }

            impl<V> crate::visitor::TraverseMut<V> for #ident
            where
                V: crate::visitor::#visitor_mut_trait_ident + ?Sized,
            {
                fn traverse_mut(&mut self, visitor: &mut V) {
                    visitor.#visit_method_ident(self);
                    visitor.#visit_method_mut_ident(self);
                    #(#children_mut)*
                }
            }
        }
    };

    impl_tokens.into()
}

#[proc_macro_attribute]
pub fn visitor(args: TokenStream, input: TokenStream) -> TokenStream {
    let args = parse_macro_input!(args as VisitorArgs);
    let item = parse_macro_input!(input as ItemStruct);

    if !item.fields.is_empty() {
        return syn::Error::new_spanned(&item.ident, "visitor marker must be a unit struct")
            .to_compile_error()
            .into();
    }

    let vis = item.vis;
    let trait_ident = item.ident;
    let trait_mut_ident = args.mut_trait;

    let methods = args
        .nodes
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
                fn #method_ident(&mut self, _node: &#node) {}
            }
        })
        .collect::<Vec<_>>();

    let mut_methods = args
        .nodes
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

            let method_mut_ident =
                format_ident!("visit_{}_mut", type_method_suffix(&type_ident.to_string()));
            quote! {
                fn #method_mut_ident(&mut self, _node: &mut #node) {}
            }
        })
        .collect::<Vec<_>>();

    quote! {
        #vis trait #trait_ident {
            #(#methods)*
        }

        #vis trait #trait_mut_ident: #trait_ident {
            #(#mut_methods)*
        }
    }
    .into()
}

struct VisitorArgs {
    nodes: Punctuated<syn::Type, Token![,]>,
    mut_trait: Ident,
}

impl Parse for VisitorArgs {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let nodes_ident: Ident = input.parse()?;
        if nodes_ident != "nodes" {
            return Err(syn::Error::new(
                nodes_ident.span(),
                "expected `nodes(...)` as the first argument",
            ));
        }

        let content;
        syn::parenthesized!(content in input);
        let nodes = content.parse_terminated(syn::Type::parse, Token![,])?;

        input.parse::<Token![,]>()?;

        let mut_trait_ident: Ident = input.parse()?;
        if mut_trait_ident != "mut_trait" {
            return Err(syn::Error::new(
                mut_trait_ident.span(),
                "expected `mut_trait = <TraitName>` as the second argument",
            ));
        }

        input.parse::<Token![=]>()?;
        let mut_trait: Ident = input.parse()?;

        if !input.is_empty() {
            return Err(input.error("unexpected tokens after `mut_trait = ...`"));
        }

        Ok(Self { nodes, mut_trait })
    }
}

#[derive(Clone, Copy)]
enum TraversalMode {
    Immutable,
    Mutable,
}

fn struct_children(fields: &Fields, mode: TraversalMode) -> Vec<proc_macro2::TokenStream> {
    match fields {
        Fields::Named(fields) => fields
            .named
            .iter()
            .filter_map(|field| {
                if !has_visit_attr(field) {
                    return None;
                }

                let field_ident = field.ident.clone().expect("named field");
                match mode {
                    TraversalMode::Immutable => Some(quote! {
                        crate::visitor::Traverse::traverse(&self.#field_ident, visitor);
                    }),
                    TraversalMode::Mutable => Some(quote! {
                        crate::visitor::TraverseMut::traverse_mut(&mut self.#field_ident, visitor);
                    }),
                }
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
                match mode {
                    TraversalMode::Immutable => Some(quote! {
                        crate::visitor::Traverse::traverse(&self.#index, visitor);
                    }),
                    TraversalMode::Mutable => Some(quote! {
                        crate::visitor::TraverseMut::traverse_mut(&mut self.#index, visitor);
                    }),
                }
            })
            .collect(),
        Fields::Unit => Vec::new(),
    }
}

fn enum_children(data_enum: &syn::DataEnum, mode: TraversalMode) -> Vec<proc_macro2::TokenStream> {
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

                    match mode {
                        TraversalMode::Immutable => quote! {
                            Self::#variant_ident { #(#all_names),* } => {
                                #(crate::visitor::Traverse::traverse(#names, visitor);)*
                            }
                        },
                        TraversalMode::Mutable => quote! {
                            Self::#variant_ident { #(#all_names),* } => {
                                #(crate::visitor::TraverseMut::traverse_mut(#names, visitor);)*
                            }
                        },
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

                    match mode {
                        TraversalMode::Immutable => quote! {
                            Self::#variant_ident(#(#names),*) => {
                                #(crate::visitor::Traverse::traverse(#traversable, visitor);)*
                            }
                        },
                        TraversalMode::Mutable => quote! {
                            Self::#variant_ident(#(#names),*) => {
                                #(crate::visitor::TraverseMut::traverse_mut(#traversable, visitor);)*
                            }
                        },
                    }
                }
                Fields::Unit => {
                    quote! { Self::#variant_ident => {} }
                }
            }
        })
        .collect::<Vec<_>>()
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
