use crate::Rule;

use super::{span::GTSpan, GTNode, GTSourceCode};
use miette::{Diagnostic, LabeledSpan, SourceCode};
use pest::error::InputLocation;
use thiserror::Error;

#[derive(Debug, PartialEq)]
pub enum GTNodeParseError {
    Internal(GTSpan, GTNode),
}

impl GTNodeParseError {
    pub fn span(&self) -> GTSpan {
        match self {
            Self::Internal(span, _) => span.clone(),
        }
    }

    pub fn message(&self) -> String {
        match self {
            Self::Internal(_, node) => format!("failed to parse {:?} node", node.name()),
        }
    }
}

#[derive(Error, Debug, PartialEq)]
#[error("Failed to parse module")]
pub struct GTModuleParseError {
    code: &'static str,
    source_code: GTSourceCode,
    message: String,
    span: GTSpan,
}

impl GTModuleParseError {
    pub fn from_pest_error(source_code: GTSourceCode, error: pest::error::Error<Rule>) -> Self {
        let message = error.variant.message().to_string();

        let span = match error.location {
            InputLocation::Pos(start) => (start, start),
            InputLocation::Span(span) => span,
        }
        .into();

        Self {
            code: "GTE001",
            source_code,
            message,
            span,
        }
    }

    pub fn from_node_error(source_code: GTSourceCode, error: GTNodeParseError) -> Self {
        Self {
            code: "GTE002",
            source_code,
            span: error.span(),
            message: format!("expected {} node", error.message()),
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

    fn help<'a>(&'a self) -> Option<Box<dyn core::fmt::Display + 'a>> {
        Some(Box::new("Hello, world!".to_string()))
    }
}
