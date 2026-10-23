use crate::prelude::internal::*;

impl GtParseToken for GtIdentifier {
    fn parse(pair: Pair<'_, Rule>) -> Self {
        GtIdentifier::new(
            (pair.as_span().start(), pair.as_span().end()).into(),
            pair.as_str().into(),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        let mut pairs = GenotypeParser::parse(Rule::name, "Hello").unwrap();
        assert_eq!(
            GtIdentifier::new((0, 5).into(), "Hello".into()),
            GtIdentifier::parse(pairs.next().unwrap()),
        );
    }
}
