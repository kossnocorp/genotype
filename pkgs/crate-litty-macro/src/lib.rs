use proc_macro::TokenStream;

mod literal;
mod literal_fields;

#[proc_macro_attribute]
pub fn literal(attr: TokenStream, item: TokenStream) -> TokenStream {
    literal::macro_attribute(attr, item)
}

#[proc_macro_attribute]
pub fn serialize_literal(attr: TokenStream, item: TokenStream) -> TokenStream {
    literal::macro_attribute_serialize_literal(attr, item)
}

#[proc_macro_attribute]
pub fn deserialize_literal(attr: TokenStream, item: TokenStream) -> TokenStream {
    literal::macro_attribute_deserialize_literal(attr, item)
}

#[proc_macro_derive(Literals, attributes(literals, literal))]
pub fn literals(item: TokenStream) -> TokenStream {
    literal_fields::macro_derive_literals(item)
}

#[proc_macro_derive(SerializeLiterals, attributes(literals, literal))]
pub fn serialize_literals(item: TokenStream) -> TokenStream {
    literal_fields::macro_derive_serialize_literals(item)
}

#[proc_macro_derive(DeserializeLiterals, attributes(literals, literal))]
pub fn deserialize_literals(item: TokenStream) -> TokenStream {
    literal_fields::macro_derive_deserialize_literals(item)
}
