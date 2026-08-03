use crate::prelude::internal::*;

#[derive(Error, Diagnostic, Debug, PartialEq, Clone, Serialize)]
pub enum GtParseError {
    /// Syntax errors collected while parsing the source.
    #[error("Syntax error")]
    #[diagnostic(code("GT001"))]
    Syntax { errors: Vec<GtSyntaxError> },

    #[error("failed to parse {1} node: {2}")]
    #[diagnostic(code("GT003"))]
    Internal(#[label("{2}")] GtSpan, GtNode, &'static str),

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
    pub fn as_diagnostic(&self, path: &str, source_code: NamedSource<String>) -> GtDiagnostic {
        match self {
            GtParseError::Syntax { errors } => {
                let labels = errors
                    .iter()
                    .map(|error| LabeledSpan::at(error.span, error.message.clone()))
                    .collect::<Vec<_>>();
                let report = miette!(
                    labels = labels,
                    "{} syntax error{}",
                    errors.len(),
                    if errors.len() == 1 { "" } else { "s" }
                )
                .with_source_code(source_code);
                GtDiagnostic {
                    kind: GtDiagnosticKind::Error,
                    content: GtDiagnosticContent::Message(GtDiagnosticContentMessage {
                        title: format!("Failed to parse module `{path}`"),
                        body: Some(vec![format!("{report:?}")].into()),
                    }),
                }
            }

            err => GtDiagnostic {
                kind: GtDiagnosticKind::Error,
                content: format!("{err}").into(),
            },
        }
    }
}

#[derive(Debug, PartialEq, Clone, Serialize)]
pub struct GtSyntaxError {
    pub span: GtSpan,
    pub message: String,
}
