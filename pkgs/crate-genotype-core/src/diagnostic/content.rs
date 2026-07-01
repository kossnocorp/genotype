use crate::prelude::internal::*;

// region: GtDiagnosticContent

impl From<(String, Vec<String>)> for GtDiagnosticContent {
    fn from((title, reports): (String, Vec<String>)) -> Self {
        GtDiagnosticContent::Message(GtDiagnosticContentMessage {
            title,
            body: Some(reports.into()),
        })
    }
}

impl<Str: AsRef<str>> From<(Str, Str)> for GtDiagnosticContent {
    fn from((title, body): (Str, Str)) -> Self {
        GtDiagnosticContent::Message(GtDiagnosticContentMessage {
            title: title.as_ref().to_string(),
            body: Some(body.as_ref().to_string().into()),
        })
    }
}

impl From<String> for GtDiagnosticContent {
    fn from(value: String) -> Self {
        GtDiagnosticContent::Message(GtDiagnosticContentMessage {
            title: value,
            body: None,
        })
    }
}

impl From<&str> for GtDiagnosticContent {
    fn from(value: &str) -> Self {
        value.to_string().into()
    }
}

impl From<Report> for GtDiagnosticContent {
    fn from(report: Report) -> Self {
        GtDiagnosticContent::Message(GtDiagnosticContentMessage {
            title: format!("{report:?}"),
            body: None,
        })
    }
}

impl From<GtDiagnostic> for Vec<GtDiagnostic> {
    fn from(val: GtDiagnostic) -> Self {
        vec![val]
    }
}

// endregion

// region: GtDiagnosticContentMessageBody

impl From<String> for GtDiagnosticContentMessageBody {
    fn from(value: String) -> Self {
        GtDiagnosticContentMessageBody::Single(value)
    }
}

impl From<Vec<String>> for GtDiagnosticContentMessageBody {
    fn from(value: Vec<String>) -> Self {
        GtDiagnosticContentMessageBody::Multi(value)
    }
}

// endregion
