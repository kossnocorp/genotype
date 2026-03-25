use crate::prelude::internal::*;

#[derive(Debug, Eq, PartialEq, Hash, Clone, Serialize, Visitor)]
pub struct GTKey(pub GTSpan, pub Arc<str>);

impl GTKey {
    pub fn new(span: GTSpan, name: Arc<str>) -> Self {
        Self(span, name)
    }
}

impl GTKey {
    pub fn parse(pair: Pair<'_, Rule>) -> Self {
        GTKey::new(pair.as_span().into(), pair.as_str().into())
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
            GTKey::new((0, 5).into(), "hello".into()),
            GTKey::parse(pairs.next().unwrap()),
        );
    }
}
