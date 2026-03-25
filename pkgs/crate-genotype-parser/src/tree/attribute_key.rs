use crate::prelude::internal::*;

#[derive(Debug, Eq, PartialEq, Hash, Clone, Serialize, Visitor)]
pub struct GTAttributeKey {
    pub span: GTSpan,
    pub value: Arc<str>,
}

impl GTAttributeKey {
    pub fn new(span: GTSpan, name: Arc<str>) -> Self {
        Self { span, value: name }
    }
}

impl GTAttributeKey {
    pub fn parse(pair: Pair<'_, Rule>) -> Self {
        GTAttributeKey::new(pair.as_span().into(), pair.as_str().into())
    }
}

#[cfg(test)]
mod tests {
    use crate::*;
    use pest::Parser;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_parse() {
        let mut pairs = GenotypeParser::parse(Rule::name, "hello").unwrap();
        assert_eq!(
            GTAttributeKey::new((0, 5).into(), "hello".into()),
            GTAttributeKey::parse(pairs.next().unwrap()),
        );
    }
}
