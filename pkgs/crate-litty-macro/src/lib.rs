use proc_macro::TokenStream;

mod literal;
mod literal_fields;

#[proc_macro_attribute]
pub fn literal(attr: TokenStream, item: TokenStream) -> TokenStream {
    literal::macro_attribute(attr, item)
}

#[proc_macro_attribute]
pub fn serde_literal(attr: TokenStream, item: TokenStream) -> TokenStream {
    literal::macro_attribute_serde_literal(attr, item)
}

#[proc_macro_attribute]
#[deprecated(note = "use #[serde_literal] with #[derive(Serialize)] instead")]
pub fn serialize_literal(attr: TokenStream, item: TokenStream) -> TokenStream {
    literal::macro_attribute_serialize_literal(attr, item)
}

#[proc_macro_attribute]
#[deprecated(note = "use #[serde_literal] with #[derive(Deserialize)] instead")]
pub fn deserialize_literal(attr: TokenStream, item: TokenStream) -> TokenStream {
    literal::macro_attribute_deserialize_literal(attr, item)
}

#[proc_macro_attribute]
pub fn serde_literals(attr: TokenStream, item: TokenStream) -> TokenStream {
    literal_fields::macro_attribute_serde_literals(attr, item)
}

#[deprecated(note = "use #[serde_literals] with #[derive(Serialize, Deserialize)] instead")]
#[proc_macro_derive(Literals, attributes(literals, literal, serde))]
pub fn literals(item: TokenStream) -> TokenStream {
    literal_fields::macro_derive_literals(item)
}

#[deprecated(note = "use #[serde_literals] with #[derive(Serialize)] instead")]
#[proc_macro_derive(SerializeLiterals, attributes(literals, literal, serde))]
pub fn serialize_literals(item: TokenStream) -> TokenStream {
    literal_fields::macro_derive_serialize_literals(item)
}

#[deprecated(note = "use #[serde_literals] with #[derive(Deserialize)] instead")]
#[proc_macro_derive(DeserializeLiterals, attributes(literals, literal, serde))]
pub fn deserialize_literals(item: TokenStream) -> TokenStream {
    literal_fields::macro_derive_deserialize_literals(item)
}
