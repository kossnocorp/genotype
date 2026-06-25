use crate::prelude::internal::*;

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct GtNotice {
    pub kind: GtNoticeKind,
    pub content: GtNoticeContent,
}

impl GtNotice {
    pub fn title(&self) -> &str {
        match &self.content {
            GtNoticeContent::Message { title, .. } => title,
            GtNoticeContent::Reports { title, .. } => title,
        }
    }

    pub fn error<Content: Into<GtNoticeContent>>(content: Content) -> Self {
        GtNotice {
            kind: GtNoticeKind::Error,
            content: content.into(),
        }
    }

    pub fn warning<Content: Into<GtNoticeContent>>(content: Content) -> Self {
        GtNotice {
            kind: GtNoticeKind::Warning,
            content: content.into(),
        }
    }

    pub fn success<Content: Into<GtNoticeContent>>(content: Content) -> Self {
        GtNotice {
            kind: GtNoticeKind::Success,
            content: content.into(),
        }
    }

    pub fn info<Content: Into<GtNoticeContent>>(content: Content) -> Self {
        GtNotice {
            kind: GtNoticeKind::Info,
            content: content.into(),
        }
    }

    pub fn format_report(report: Report) -> String {
        format!("{report:?}")
    }
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub enum GtNoticeKind {
    Error,
    Warning,
    Success,
    Info,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub enum GtNoticeContent {
    Message { title: String, body: Option<String> },
    Reports { title: String, reports: Vec<String> },
}

impl From<(String, Vec<String>)> for GtNoticeContent {
    fn from((title, reports): (String, Vec<String>)) -> Self {
        GtNoticeContent::Reports { title, reports }
    }
}

impl From<(String, String)> for GtNoticeContent {
    fn from((title, body): (String, String)) -> Self {
        GtNoticeContent::Message {
            title,
            body: Some(body),
        }
    }
}

impl From<String> for GtNoticeContent {
    fn from(value: String) -> Self {
        GtNoticeContent::Message {
            title: value,
            body: None,
        }
    }
}

impl From<&str> for GtNoticeContent {
    fn from(value: &str) -> Self {
        value.to_string().into()
    }
}

impl From<Report> for GtNoticeContent {
    fn from(report: Report) -> Self {
        GtNoticeContent::Message {
            title: format!("{report:?}"),
            body: None,
        }
    }
}

impl From<GtNotice> for Vec<GtNotice> {
    fn from(val: GtNotice) -> Self {
        vec![val]
    }
}

// pub fn message(title: impl Into<String>, body: impl Into<String>) -> GtNoticeContent {
//     GtNoticeContent::Message {
//         title: title.into(),
//         body: Some(body.into()),
//     }
// }

// pub fn reports(title: impl Into<String>, reports: Vec<String>) -> GtNoticeContent {
//     GtNoticeContent::Reports {
//         title: title.into(),
//         reports,
//     }
// }

// pub trait GtpNoticeable {
//     fn as_notice(&self) -> Option<GtpNotice> {
//         Some(GtpNotice {
//             kind: self.kind(),
//             message: self.format(),
//         })
//     }

//     fn kind(&self) -> GtpNoticeKind;

//     fn format(&self) -> String;
// }

// pub trait GtNoticeableCollection {
//     fn as_notices(&self) -> Vec<GtNotice>;
// }

// pub trait GtpNoticeableError {
//     fn format_error(&self) -> String {
//         let report = self.report();
//         format!("{report}")
//     }

//     fn report(&self) -> Report;
// }

// impl<Type: GtpNoticeableError> GtpNoticeable for Type {
//     fn format(&self) -> String {
//         self.format_error()
//     }

//     fn kind(&self) -> GtpNoticeKind {
//         GtpNoticeKind::Error
//     }
// }
