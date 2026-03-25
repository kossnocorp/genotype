use crate::prelude::internal::*;

#[derive(Debug, Eq, PartialEq, Hash, Clone, Serialize, Visitor)]
pub struct GTAttributeName {
    pub span: GTSpan,
    pub value: Arc<str>,
}

impl GTAttributeName {
    pub fn new(span: GTSpan, value: Arc<str>) -> Self {
        Self { span, value }
    }
}

impl GTAttributeName {
    pub fn parse(pair: Pair<'_, Rule>) -> Self {
        GTAttributeName::new(pair.as_span().into(), pair.as_str().into())
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
            GTAttributeName::new((0, 5).into(), "hello".into()),
            GTAttributeName::parse(pairs.next().unwrap()),
        );
    }
}
