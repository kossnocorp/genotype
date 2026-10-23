use crate::prelude::internal::*;

#[derive(Error, Diagnostic, Debug, PartialEq, Clone, Serialize)]
pub enum GtParseError {
    #[error(transparent)]
    #[diagnostic(transparent)]
    Descriptor(#[from] GtDescriptorError),

    /// Syntax error with a source span and miette diagnostic label.
    #[error("Syntax error")]
    #[diagnostic(code("GT001"))]
    Syntax {
        #[label("{message}")]
        span: GtSpan,
        message: String,
    },

    /// Parsing succeeded but the result didn't match the expected structure.
    #[error("Invalid grammar")]
    #[diagnostic(code("GT002"))]
    InvalidGrammar,

    #[error("failed to parse {1} node: {2}")]
    #[diagnostic(code("GT003"))]
    Internal(#[label("{2}")] GtSpan, GtNode, &'static str),

    #[error("Encountered unexpected token '{2}' while converting '{1}' node: {3}")]
    #[diagnostic(code("GT004"))]
    UnexpectedToken(
        #[label("{3}")] GtSpan,
        GtNode,
        #[serde(serialize_with = "serialize_token_kind")] GtTokenKind,
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
}

impl GtParseError {
    pub fn as_diagnostic(&self, path: &str, source_code: NamedSource<String>) -> GtDiagnostic {
        match self {
            GtParseError::Syntax { span, message } => {
                let report = miette!(
                    labels = vec![LabeledSpan::at(*span, "Here")],
                    "Syntax error: {message}"
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

fn serialize_token_kind<S>(kind: &GtTokenKind, serializer: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    serializer.serialize_str(kind.as_str())
}
