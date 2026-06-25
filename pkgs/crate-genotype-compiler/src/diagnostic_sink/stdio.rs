use crate::prelude::internal::*;

use owo_colors::OwoColorize;

pub trait GtcDiagnosticSinkStdio {
    fn print_diagnostic_title(kind: &GtDiagnosticKind, message: &str) {
        match kind {
            GtDiagnosticKind::Success => {
                println!("{label}: {message}.", label = "Success".green().bold())
            }

            GtDiagnosticKind::Info => println!("{label}: {message}.", label = "Info".blue().bold()),

            GtDiagnosticKind::Warning => {
                eprintln!("{label}: {message}.", label = "Warning".yellow().bold())
            }

            GtDiagnosticKind::Error => {
                eprintln!("{label}: {message}.", label = "Error".red().bold())
            }
        }
    }

    fn print_diagnostic_content(diagnostic: &GtDiagnostic) {
        match &diagnostic.content {
            GtDiagnosticContent::Message(message) => match message.body.as_ref() {
                Some(GtDiagnosticContentMessageBody::Single(body)) => {
                    println!();
                    Self::print_diagnostic_body(&diagnostic.kind, &body);
                }

                Some(GtDiagnosticContentMessageBody::Multi(bodies)) => {
                    println!();
                    for body in bodies {
                        Self::print_diagnostic_body(&diagnostic.kind, &body);
                    }
                }

                None => {}
            },

            GtDiagnosticContent::Report(report) => {
                todo!("Implement report printing for GtDiagnosticContent::Report: {report:?}")
            }
        }
    }

    fn print_diagnostic_body(kind: &GtDiagnosticKind, body: &str) {
        let body = body.trim();
        match kind {
            GtDiagnosticKind::Success | GtDiagnosticKind::Info => {
                println!("{body}")
            }
            GtDiagnosticKind::Warning | GtDiagnosticKind::Error => {
                eprintln!("{body}")
            }
        }
    }
}

impl<Type: GtcDiagnosticSinkStdio> GtcDiagnosticSink for Type {
    fn print_diagnostic(&self, diagnostic: GtDiagnostic) {
        Self::print_diagnostic_title(&diagnostic.kind, diagnostic.title());
        Self::print_diagnostic_content(&diagnostic);

        println!();
    }
}
