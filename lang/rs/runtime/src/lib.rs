use proc_macro::TokenStream;

mod literal;
mod union;

#[proc_macro_attribute]
pub fn literal(attr: TokenStream, item: TokenStream) -> TokenStream {
    literal::macro_attribute(attr, item)
}

#[proc_macro_attribute]
pub fn union(attr: TokenStream, item: TokenStream) -> TokenStream {
    union::macro_attribute(attr, item)
}
