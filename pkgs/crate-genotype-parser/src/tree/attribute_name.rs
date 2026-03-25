use crate::prelude::internal::*;

#[derive(Debug, Eq, PartialEq, Hash, Clone, Serialize, Visitor)]
pub struct GtAttributeName {
    pub span: GtSpan,
    pub value: Arc<str>,
}

impl GtAttributeName {
    pub fn new(span: GtSpan, value: Arc<str>) -> Self {
        Self { span, value }
    }
}

impl GtAttributeName {
    pub fn parse(pair: Pair<'_, Rule>) -> Self {
        GtAttributeName::new(pair.as_span().into(), pair.as_str().into())
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
            GtAttributeName::new((0, 5).into(), "hello".into()),
            GtAttributeName::parse(pairs.next().unwrap()),
        );
    }
}
