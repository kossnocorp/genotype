use crate::prelude::internal::*;

#[derive(Error, Diagnostic, Debug, PartialEq)]
pub enum GtParseError {
    #[error("Failed to parse {1} node")]
    #[diagnostic(code("GTP001"))]
    Internal(#[label("internal error")] GtSpan, GtNode),

    #[error("Failed to parse {1} node")]
    #[diagnostic(code("GTP002"))]
    InternalMessage(#[label("{2}")] GtSpan, GtNode, &'static str),

    #[error("Encountered unexpected rule '{2:?}' while parsing '{1}' node")]
    #[diagnostic(code("GTP003"))]
    UnexpectedRule(#[label("unexpected rule")] GtSpan, GtNode, Rule),

    #[error("Failed to parse {1} node")]
    #[diagnostic(code("GTP004"))]
    UnexpectedEnd(#[label("unexpected end")] GtSpan, GtNode),

    #[error("Failed to parse {1} node")]
    #[diagnostic(code("GTP005"))]
    UnknownValue(#[label("unknown value")] GtSpan, GtNode),

    #[error("Failed to extract expected type from descriptor")]
    UnmatchedDescriptor(#[label("incorrect type descriptor")] GtSpan, GtNode),
}

impl GtParseError {
    pub fn span(&self) -> GtSpan {
        match self {
            Self::Internal(span, _) => *span,
            Self::InternalMessage(span, _, _) => *span,
            Self::UnexpectedRule(span, _, _) => *span,
            Self::UnexpectedEnd(span, _) => *span,
            Self::UnknownValue(span, _) => *span,
            Self::UnmatchedDescriptor(span, _) => *span,
        }
    }

    pub fn message(&self) -> String {
        match self {
            Self::Internal(_, node) => format!("failed to parse {:?} node", node.name()),
            Self::InternalMessage(_, node, message) => {
                format!("failed to parse {:?} node: {}", node.name(), message)
            }
            Self::UnexpectedRule(_, node, rule) => {
                format!(
                    "failed to parse {:?} node: unexpected rule {:?}",
                    node.name(),
                    rule
                )
            }
            Self::UnexpectedEnd(_, node) => {
                format!("failed to parse {:?} node: unexpected end", node.name())
            }
            Self::UnknownValue(_, node) => {
                format!("failed to parse {:?} node: unknown value", node.name())
            }
            Self::UnmatchedDescriptor(_, node) => {
                format!(
                    "failed to extract expected type from {:?} descriptor",
                    node.name()
                )
            }
        }
    }
}

#[derive(Error, Debug, PartialEq)]
#[error("Failed to parse module")]
pub struct GtModuleParseError {
    code: &'static str,
    source_code: NamedSource<String>,
    message: String,
    span: GtSpan,
}

impl GtModuleParseError {
    pub fn from_pest_error(
        source_code: NamedSource<String>,
        error: pest::error::Error<Rule>,
    ) -> Self {
        let message = error.variant.message().to_string();

        let span = match error.location {
            InputLocation::Pos(start) => (start, start),
            InputLocation::Span(span) => span,
        }
        .into();

        Self {
            code: "GTP001",
            source_code,
            message,
            span,
        }
    }

    pub fn from_node_error(source_code: NamedSource<String>, error: GtParseError) -> Self {
        Self {
            code: "GTP002",
            source_code,
            span: error.span(),
            message: error.message(),
        }
    }
}

impl Diagnostic for GtModuleParseError {
    fn source_code(&self) -> Option<&dyn SourceCode> {
        Some(&self.source_code)
    }

    fn labels(&self) -> Option<Box<dyn Iterator<Item = LabeledSpan>>> {
        Some(Box::new(std::iter::once(LabeledSpan::new(
            Some(self.message.clone()),
            self.span.offset(),
            self.span.len(),
        ))))
    }

    fn code<'a>(&'a self) -> Option<Box<dyn std::fmt::Display + 'a>> {
        Some(Box::new(self.code))
    }

    // fn help<'a>(&'a self) -> Option<Box<dyn core::fmt::Display + 'a>> {
    //     Some(Box::new("Hello, world!".to_string()))
    // }
}
