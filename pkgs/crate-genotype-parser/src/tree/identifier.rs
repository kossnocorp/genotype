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
        self.0.clone()
    }

    pub fn as_str(&self) -> &str {
        self.1.as_ref()
    }

    pub fn as_string(&self) -> String {
        self.1.to_string()
    }
}

impl From<Pair<'_, Rule>> for GtIdentifier {
    fn from(pair: Pair<'_, Rule>) -> Self {
        GtIdentifier::new(pair.as_span().into(), pair.as_str().into())
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
            GtIdentifier::new((0, 5).into(), "Hello".into()),
            pairs.next().unwrap().into(),
        );
    }
}
