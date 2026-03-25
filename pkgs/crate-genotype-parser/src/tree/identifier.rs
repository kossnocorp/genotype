use crate::prelude::internal::*;

/// Unique module identifier.
#[derive(Debug, Eq, PartialEq, Hash, Clone, Serialize, Visitor)]
pub struct GTIdentifier(
    /// Identifier position in the source code.
    pub GTSpan,
    /// Identifier name.
    pub Arc<str>,
);

impl GTIdentifier {
    pub fn new(span: GTSpan, name: Arc<str>) -> Self {
        Self(span, name)
    }

    pub fn as_span(&self) -> GTSpan {
        self.0.clone()
    }

    pub fn as_str(&self) -> &str {
        self.1.as_ref()
    }

    pub fn as_string(&self) -> String {
        self.1.to_string()
    }
}

impl From<Pair<'_, Rule>> for GTIdentifier {
    fn from(pair: Pair<'_, Rule>) -> Self {
        GTIdentifier::new(pair.as_span().into(), pair.as_str().into())
    }
}

#[cfg(test)]
mod tests {
    use crate::*;
    use pest::Parser;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_parse() {
        let mut pairs = GenotypeParser::parse(Rule::name, "Hello").unwrap();
        assert_eq!(
            GTIdentifier::new((0, 5).into(), "Hello".into()),
            pairs.next().unwrap().into(),
        );
    }
}
