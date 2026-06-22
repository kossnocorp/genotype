use crate::prelude::internal::*;

use owo_colors::OwoColorize;

pub trait GtcNoticeSinkStdio {
    fn print_notice_title(kind: &GtNoticeKind, message: &str) {
        match kind {
            GtNoticeKind::Success => {
                println!("{label}: {message}.", label = "Success".green().bold())
            }
            GtNoticeKind::Info => println!("{label}: {message}.", label = "Info".blue().bold()),
            GtNoticeKind::Warning => {
                eprintln!("{label}: {message}.", label = "Warning".yellow().bold())
            }
            GtNoticeKind::Error => eprintln!("{label}: {message}.", label = "Error".red().bold()),
        }
    }

    fn print_notice_reports(notice: &GtNotice) {
        match &notice.content {
            GtNoticeContent::Reports { reports, .. } => {
                println!();
                for report in reports {
                    let report = report.trim();
                    match &notice.kind {
                        GtNoticeKind::Success | GtNoticeKind::Info => println!("{report}"),
                        GtNoticeKind::Warning | GtNoticeKind::Error => eprintln!("{report}"),
                    }
                }
            }
            _ => {}
        }
    }
}

impl<Type: GtcNoticeSinkStdio> GtcNoticeSink for Type {
    fn print_notice(&self, notice: GtNotice) {
        Self::print_notice_title(&notice.kind, notice.title());
        Self::print_notice_reports(&notice);

        println!();
    }
}
