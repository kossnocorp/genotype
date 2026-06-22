use crate::prelude::internal::*;

#[derive(Error, Diagnostic, Debug, PartialEq, Clone, Serialize)]
pub enum GtParseError {
    /// Pest syntax error.
    #[error("Syntax error")]
    #[diagnostic(code("GT001"))]
    Syntax {
        #[label("{message}")]
        span: GtSpan,
        message: String,
    },

    /// Pest succeeded but the result didn't match the expected structure.
    #[error("Invalid grammar")]
    #[diagnostic(code("GT002"))]
    InvalidGrammar,

    #[error("failed to parse {1} node: {2}")]
    #[diagnostic(code("GT003"))]
    Internal(#[label("{2}")] GtSpan, GtNode, &'static str),

    #[error("Encountered unexpected rule '{2:?}' while parsing '{1}' node: {3}")]
    #[diagnostic(code("GT004"))]
    UnexpectedRule(
        #[label("{3}")] GtSpan,
        GtNode,
        #[serde(serialize_with = "serialize_rule")] Rule,
        &'static str,
    ),

    #[error("failed to parse {1} node")]
    #[diagnostic(code("GT005"))]
    UnexpectedEnd(
        #[label("unexpected end; expected {2}")] GtSpan,
        GtNode,
        &'static str,
    ),

    #[error("Failed to parse {1} node")]
    #[diagnostic(code("GT006"))]
    UnknownValue(#[label("unknown value")] GtSpan, GtNode),

    #[error("Failed to extract expected type from descriptor")]
    #[diagnostic(code("GT007"))]
    UnmatchedDescriptor(#[label("incorrect type descriptor")] GtSpan, GtNode),
}

impl GtParseError {
    pub fn as_notice(&self, path: &str, source_code: NamedSource<String>) -> GtNotice {
        match self {
            GtParseError::Syntax { span, message } => {
                let report = miette!(
                    labels = vec![LabeledSpan::at(span.clone(), "Here")],
                    "Syntax error: {message}"
                )
                .with_source_code(source_code);
                GtNotice {
                    kind: GtNoticeKind::Error,
                    content: GtNoticeContent::Reports {
                        title: format!("Failed to parse module `{path}`"),
                        reports: vec![format!("{report:?}")],
                    },
                }
            }

            err => GtNotice {
                kind: GtNoticeKind::Error,
                content: format!("{err}").into(),
            },
        }
    }
}

impl Into<GtParseError> for pest::error::Error<Rule> {
    fn into(self) -> GtParseError {
        let span = match self.location {
            InputLocation::Pos(pos) => (pos, pos).into(),
            InputLocation::Span((start, end)) => (start, end).into(),
        };
        let message = self.variant.message().to_string();
        GtParseError::Syntax { span, message }
    }
}

fn serialize_rule<S>(rule: &Rule, serializer: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    serializer.serialize_str(&format!("{rule:?}"))
}

impl GtParseError {
    pub fn span(&self) -> GtSpan {
        match self {
            Self::Internal(span, _, _) => *span,
            Self::UnexpectedRule(span, _, _, _) => *span,
            Self::UnexpectedEnd(span, _, _) => *span,
            Self::UnknownValue(span, _) => *span,
            Self::UnmatchedDescriptor(span, _) => *span,
            _ => todo!("Get rid of GtModuleParseError"),
        }
    }

    pub fn message(&self) -> String {
        match self {
            Self::Internal(_, node, message) => {
                format!("failed to parse {:?} node: {}", node.name(), message)
            }
            Self::UnexpectedRule(_, node, rule, message) => {
                format!(
                    "failed to parse {:?} node: unexpected rule {:?}: {}",
                    node.name(),
                    rule,
                    message
                )
            }
            Self::UnexpectedEnd(_, node, message) => {
                format!("failed to parse {:?} node: {}", node.name(), message)
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
