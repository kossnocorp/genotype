use proc_macro::TokenStream;

mod literal;

#[proc_macro_attribute]
pub fn literal(attr: TokenStream, item: TokenStream) -> TokenStream {
    literal::macro_attribute(attr, item)
}
