use crate::prelude::internal::*;

pub trait GtTestContext {
    fn context() -> GtContext;
}

impl GtTestContext for genotype_tree::test::Gt {
    fn context() -> GtContext {
        GtContext::new("module".into())
    }
}

pub fn to_parse_args(kind: GtTokenKind, code: &str) -> (GtTokens<'_>, GtContext) {
    let tokens = GtParser::parse(kind, code).unwrap_or_else(|e| {
        panic!("Failed to parse code:\n{}\nError: {}", code, e);
    });
    let context = GtContext::new("module".into());
    (tokens, context)
}

pub fn to_parse_tokens(kind: GtTokenKind, code: &str) -> GtTokens<'_> {
    GtParser::parse(kind, code).unwrap_or_else(|e| {
        panic!("Failed to parse code:\n{}\nError: {}", code, e);
    })
}

#[macro_export]
macro_rules! parse_node {
    ($ty:ty, $args:expr) => {{
        let (mut tokens, mut context) = $args;
        <$ty>::parse(tokens.next().unwrap(), &mut context).unwrap()
    }};
}

#[macro_export]
macro_rules! parse_node_err {
    ($ty:ty, $args:expr) => {{
        let (mut tokens, mut context) = $args;
        <$ty>::parse(tokens.next().unwrap(), &mut context).unwrap_err()
    }};
}
