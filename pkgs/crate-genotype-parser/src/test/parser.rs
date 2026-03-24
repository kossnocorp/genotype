use crate::prelude::internal::*;

pub fn to_parse_args(rule: Rule, code: &str) -> (Pairs<'_, Rule>, GTContext) {
    let pairs = GenotypeParser::parse(rule, code).unwrap_or_else(|e| {
        panic!("Failed to parse code:\n{}\nError: {}", code, e);
    });
    let context = GTContext::new("module".into());
    (pairs, context)
}

pub fn to_parse_rules(rule: Rule, code: &str) -> Pairs<'_, Rule> {
    GenotypeParser::parse(rule, code).unwrap_or_else(|e| {
        panic!("Failed to parse code:\n{}\nError: {}", code, e);
    })
}

#[macro_export]
macro_rules! parse_node {
    ($ty:ty, $args:expr) => {{
        let (mut pairs, mut context) = $args;
        <$ty>::parse(pairs.next().unwrap(), &mut context).unwrap()
    }};
}

#[macro_export]
macro_rules! parse_node_err {
    ($ty:ty, $args:expr) => {{
        let (mut pairs, mut context) = $args;
        <$ty>::parse(pairs.next().unwrap(), &mut context).unwrap_err()
    }};
}
