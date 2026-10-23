use crate::prelude::internal::*;

impl GtParseToken for GtKey {
    fn parse(token: GtToken<'_>) -> Self {
        GtKey::new(token.span, token.as_str().into())
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
            GtKey::new((0, 5).into(), "hello".into()),
            GtKey::parse(tokens.next().unwrap()),
        );
    }
}
