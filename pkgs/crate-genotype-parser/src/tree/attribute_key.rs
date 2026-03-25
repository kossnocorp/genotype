use crate::prelude::internal::*;

#[derive(Debug, Eq, PartialEq, Hash, Clone, Serialize, Visitor)]
pub struct GtAttributeKey {
    pub span: GtSpan,
    pub value: Arc<str>,
}

impl GtAttributeKey {
    pub fn new(span: GtSpan, name: Arc<str>) -> Self {
        Self { span, value: name }
    }
}

impl GtAttributeKey {
    pub fn parse(pair: Pair<'_, Rule>) -> Self {
        GtAttributeKey::new(pair.as_span().into(), pair.as_str().into())
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
            GtAttributeKey::new((0, 5).into(), "hello".into()),
            GtAttributeKey::parse(pairs.next().unwrap()),
        );
    }
}
