use crate::prelude::internal::*;

impl GtParseToken for GtIdentifier {
    fn parse(token: GtToken<'_>) -> Self {
        GtIdentifier::new(token.span, token.as_str().into())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        let mut tokens = GtParser::parse(GtTokenKind::Name, "Hello").unwrap();
        assert_eq!(
            GtIdentifier::new((0, 5).into(), "Hello".into()),
            GtIdentifier::parse(tokens.next().unwrap()),
        );
    }
}
