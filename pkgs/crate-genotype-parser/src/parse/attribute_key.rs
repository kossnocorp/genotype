use crate::prelude::internal::*;

impl GtParseToken for GtAttributeKey {
    fn parse(pair: Pair<'_, Rule>) -> Self {
        GtAttributeKey::new(
            (pair.as_span().start(), pair.as_span().end()).into(),
            pair.as_str().into(),
        )
    }
}

#[cfg(test)]
mod tests {
    use crate::prelude::internal::*;
    use pest::Parser;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_parse() {
        let mut pairs = GenotypeParser::parse(Rule::name, "hello").unwrap();
        assert_eq!(
            GtAttributeKey::new((0, 5).into(), "hello".into()),
            GtAttributeKey::parse(pairs.next().unwrap()),
        );
    }
}
