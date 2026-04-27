use crate::prelude::internal::*;

#[derive(Error, Diagnostic, Debug, PartialEq, Clone, Serialize)]
pub enum GtParseError {
    /// Pest syntax error.
    #[error("Syntax error")]
    #[diagnostic(code("GT001"))]
    Syntax(GtPestError),

    /// Pest succeeded but the result didn't match the expected structure.
    #[error("Invalid grammar")]
    #[diagnostic(code("GT002"))]
    InvalidGrammar,

    #[error("Failed to parse {1} node")]
    #[diagnostic(code("GT002"))]
    Internal(#[label("internal error")] GtSpan, GtNode),

    #[error("Failed to parse {1} node")]
    #[diagnostic(code("GT003"))]
    InternalMessage(#[label("{2}")] GtSpan, GtNode, &'static str),

    #[error("Encountered unexpected rule '{2:?}' while parsing '{1}' node")]
    #[diagnostic(code("GT004"))]
    UnexpectedRule(
        #[label("unexpected rule")] GtSpan,
        GtNode,
        #[serde(serialize_with = "serialize_rule")] Rule,
    ),

    #[error("Failed to parse {1} node")]
    #[diagnostic(code("GT005"))]
    UnexpectedEnd(#[label("unexpected end")] GtSpan, GtNode),

    #[error("Failed to parse {1} node")]
    #[diagnostic(code("GT006"))]
    UnknownValue(#[label("unknown value")] GtSpan, GtNode),

    #[error("Failed to extract expected type from descriptor")]
    #[diagnostic(code("GT007"))]
    UnmatchedDescriptor(#[label("incorrect type descriptor")] GtSpan, GtNode),
}

#[derive(Debug, PartialEq, Clone)]
pub struct GtPestError(pub pest::error::Error<Rule>);

impl From<pest::error::Error<Rule>> for GtPestError {
    fn from(error: pest::error::Error<Rule>) -> Self {
        Self(error)
    }
}

impl serde::Serialize for GtPestError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        PestErrorSnapshot::from(&self.0).serialize(serializer)
    }
}

fn serialize_rule<S>(rule: &Rule, serializer: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    serializer.serialize_str(&format!("{rule:?}"))
}

#[derive(Debug, Serialize)]
struct PestErrorSnapshot {
    message: String,
    span_start: usize,
    span_end: usize,
    line_start: usize,
    column_start: usize,
    line_end: usize,
    column_end: usize,
    positives: Option<Vec<String>>,
    negatives: Option<Vec<String>>,
    custom_message: Option<String>,
}

impl From<&pest::error::Error<Rule>> for PestErrorSnapshot {
    fn from(error: &pest::error::Error<Rule>) -> Self {
        use pest::error::{ErrorVariant, InputLocation, LineColLocation};

        let (span_start, span_end) = match error.location {
            InputLocation::Pos(start) => (start, start),
            InputLocation::Span((start, end)) => (start, end),
        };

        let (line_start, line_end) = match error.line_col {
            LineColLocation::Pos(line_col) => (line_col, line_col),
            LineColLocation::Span(start, end) => (start, end),
        };

        let (positives, negatives, custom_message) = match &error.variant {
            ErrorVariant::ParsingError {
                positives,
                negatives,
            } => (
                Some(positives.iter().map(|rule| format!("{rule:?}")).collect()),
                Some(negatives.iter().map(|rule| format!("{rule:?}")).collect()),
                None,
            ),
            ErrorVariant::CustomError { message } => (None, None, Some(message.clone())),
        };

        Self {
            message: error.variant.message().to_string(),
            span_start,
            span_end,
            line_start: line_start.0,
            column_start: line_start.1,
            line_end: line_end.0,
            column_end: line_end.1,
            positives,
            negatives,
            custom_message,
        }
    }
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
            _ => todo!("Get rid of GtModuleParseError"),
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
            _ => todo!("Get rid of GtModuleParseError"),
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
