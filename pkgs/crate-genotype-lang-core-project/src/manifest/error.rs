use crate::prelude::internal::*;

#[derive(Debug, Clone, Error, Diagnostic)]
pub enum GtlManifestError {
    #[error("Failed to parse {which} TOML")]
    TomlParse {
        which: &'static str,
        #[label("Here")]
        span: Option<GtSpan>,
        #[source_code]
        source_code: String,
        #[source]
        error: toml_edit::TomlError,
    },

    #[error("Failed to edit TOML")]
    TomlEdit {
        #[source]
        error: TomlExtError,
    },

    #[error("Failed to format JSON: {message}")]
    FormatJson {
        message: String,
        #[label("Here")]
        span: Option<GtSpan>,
        #[source_code]
        source_code: String,
    },
}

impl GtlError for GtlManifestError {
    fn clone_box(&self) -> Box<dyn GtlError> {
        Box::new(self.clone())
    }
}

impl GtlManifestError {
    pub fn toml_parse(
        which: &'static str,
        error: toml_edit::TomlError,
        source_code: String,
    ) -> Box<dyn GtlError> {
        Box::new(Self::TomlParse {
            which,
            span: error.span().map(|span| span.into()),
            source_code,
            error,
        })
    }

    pub fn edit(error: TomlExtError) -> Box<dyn GtlError> {
        Box::new(Self::TomlEdit { error })
    }

    pub fn format_json(error: serde_json::Error, source_code: String) -> Box<dyn GtlError> {
        let span = Self::line_col_to_span(error.line(), error.column(), &source_code);
        let message = format!("{error}");
        Box::new(Self::FormatJson {
            message,
            span,
            source_code,
        })
    }

    fn line_col_to_span(line: usize, col: usize, source: &str) -> Option<GtSpan> {
        use line_index::{LineCol, LineIndex};
        let index = LineIndex::new(source);
        let pos = index.offset(LineCol {
            line: line as u32,
            col: col as u32,
        })?;
        Some(GtSpan(pos.into(), pos.into()))
    }
}
