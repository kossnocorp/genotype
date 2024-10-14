use pest::iterators::Pair;

use crate::parser::Rule;

use super::GTIdentifier;

impl From<Pair<'_, Rule>> for GTIdentifier {
    fn from(pair: Pair<'_, Rule>) -> Self {
        GTIdentifier::new(pair.as_span().into(), pair.as_str().into())
    }
}

#[cfg(test)]
mod tests {
    use crate::*;
    use pest::Parser;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_parse() {
        let mut pairs = GenotypeParser::parse(Rule::name, "Hello").unwrap();
        assert_eq!(
            GTIdentifier::new((0, 5).into(), "Hello".into()),
            pairs.next().unwrap().into(),
        );
    }
}
