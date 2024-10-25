use crate::Rule;

use super::{span::GTSpan, GTNode};
use miette::{Diagnostic, Error, LabeledSpan, NamedSource, SourceCode};
use pest::error::InputLocation;
use thiserror::Error;

#[derive(Error, Diagnostic, Debug, PartialEq)]
pub enum GTParseError {
    #[error("Failed to parse {1} node")]
    #[diagnostic(code("GTP002"))]
    Internal(#[label("internal error")] GTSpan, GTNode),

    #[error("Failed to parse {1} node")]
    #[diagnostic(code("GTP002"))]
    InternalMessage(#[label("{2}")] GTSpan, GTNode, &'static str),

    #[error("Failed to parse {1} node")]
    #[diagnostic(code("GTP003"))]
    UnknownRule(#[label("unknown rule")] GTSpan, GTNode),

    #[error("Failed to parse {1} node")]
    #[diagnostic(code("GTP004"))]
    UnexpectedEnd(#[label("unexpected end")] GTSpan, GTNode),

    #[error("Failed to parse {1} node")]
    #[diagnostic(code("GTP005"))]
    UnknownValue(#[label("unknown value")] GTSpan, GTNode),
}

impl GTParseError {
    pub fn span(&self) -> GTSpan {
        match self {
            Self::Internal(span, _) => span.clone(),
            Self::InternalMessage(span, _, _) => span.clone(),
            Self::UnknownRule(span, _) => span.clone(),
            Self::UnexpectedEnd(span, _) => span.clone(),
            Self::UnknownValue(span, _) => span.clone(),
        }
    }

    pub fn message(&self) -> String {
        match self {
            Self::Internal(_, node) => format!("failed to parse {:?} node", node.name()),
            Self::InternalMessage(_, node, message) => {
                format!("failed to parse {:?} node: {}", node.name(), message)
            }
            Self::UnknownRule(_, node) => {
                format!("failed to parse {:?} node: unknown rule", node.name())
            }
            Self::UnexpectedEnd(_, node) => {
                format!("failed to parse {:?} node: unexpected end", node.name())
            }
            Self::UnknownValue(_, node) => {
                format!("failed to parse {:?} node: unknown value", node.name())
            }
        }
    }
}

#[derive(Error, Debug, PartialEq)]
#[error("Failed to parse module")]
pub struct GTModuleParseError {
    code: &'static str,
    source_code: NamedSource<String>,
    message: String,
    span: GTSpan,
}

impl GTModuleParseError {
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

    pub fn from_node_error(source_code: NamedSource<String>, error: GTParseError) -> Self {
        Self {
            code: "GTP002",
            source_code,
            span: error.span(),
            message: error.message(),
        }
    }
}

impl Diagnostic for GTModuleParseError {
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
