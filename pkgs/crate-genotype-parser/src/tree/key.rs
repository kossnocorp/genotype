use crate::prelude::internal::*;

#[derive(Debug, Eq, PartialEq, Hash, Clone, Serialize, Visitor)]
pub struct GtKey(pub GtSpan, pub Arc<str>);

impl GtKey {
    pub fn new(span: GtSpan, name: Arc<str>) -> Self {
        Self(span, name)
    }
}

impl GtKey {
    pub fn parse(pair: Pair<'_, Rule>) -> Self {
        GtKey::new(pair.as_span().into(), pair.as_str().into())
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
            GtKey::new((0, 5).into(), "hello".into()),
            GtKey::parse(pairs.next().unwrap()),
        );
    }
}
