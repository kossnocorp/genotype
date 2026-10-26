//! Language-independent document construction. Layout rules live in `format.rs`.

mod el;
pub use el::*;

/// A Genotype formatting document, borrowing its source text.
/// Construction guarantees balanced groups and indentation, so printing is infallible.
#[derive(Debug, Clone)]
pub struct GtfmtDoc<'a> {
    elements: GtfmtElements<'a>,
    source_len: usize,
}

impl<'a> GtfmtDoc<'a> {
    pub fn new(elements: GtfmtElements<'a>, source_len: usize) -> Self {
        Self {
            elements,
            source_len,
        }
    }

    /// Print with Genotype's standard width and indentation.
    pub fn print(&self) -> String {
        crate::printer::GtfmtPrinter::print(
            &self.elements,
            crate::GTFMT_PRINT_WIDTH as usize,
            self.source_len,
        )
    }
}
