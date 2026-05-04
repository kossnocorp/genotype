use crate::prelude::internal::*;

/// Unique module identifier.
#[derive(Debug, Eq, PartialEq, Hash, Clone, Serialize, Visitor)]
pub struct GtIdentifier(
    /// Identifier position in the source code.
    pub GtSpan,
    /// Identifier name.
    pub Arc<str>,
);

impl GtIdentifier {
    pub fn new(span: GtSpan, name: Arc<str>) -> Self {
        Self(span, name)
    }

    pub fn as_span(&self) -> GtSpan {
        self.0
    }

    pub fn as_str(&self) -> &str {
        self.1.as_ref()
    }

    pub fn as_string(&self) -> String {
        self.1.to_string()
    }

    pub fn has_same_name(&self, other: &GtIdentifier) -> bool {
        self.1 == other.1
    }
}

impl From<Pair<'_, Rule>> for GtIdentifier {
    fn from(pair: Pair<'_, Rule>) -> Self {
        GtIdentifier::new(pair.as_span().into(), pair.as_str().into())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        let mut pairs = GenotypeParser::parse(Rule::name, "Hello").unwrap();
        assert_eq!(
            GtIdentifier::new((0, 5).into(), "Hello".into()),
            pairs.next().unwrap().into(),
        );
    }

    #[test]
    fn test_same_name() {
        let id1 = Gt::identifier_with_span("Hello", (0, 1));
        let id2 = Gt::identifier_with_span("Hello", (0, 2));
        let id3 = Gt::identifier_with_span("World", (0, 3));
        assert!(id1.has_same_name(&id2));
        assert!(!id1.has_same_name(&id3));
    }
}
