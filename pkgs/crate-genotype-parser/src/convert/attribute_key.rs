use crate::prelude::internal::*;

impl GtParseToken for GtAttributeKey {
    fn parse(token: GtToken<'_>) -> Self {
        GtAttributeKey::new(token.span, token.as_str().into())
    }
}

#[cfg(test)]
mod tests {
    use crate::prelude::internal::*;

    use pretty_assertions::assert_eq;

    #[test]
    fn test_parse() {
        let mut tokens = GtParser::parse(GtTokenKind::Name, "hello").unwrap();
        assert_eq!(
            GtAttributeKey::new((0, 5).into(), "hello".into()),
            GtAttributeKey::parse(tokens.next().unwrap()),
        );
    }
}
