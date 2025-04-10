#[derive(Debug, Clone, PartialEq)]
pub struct GtlIndentLegacy<'a> {
    symbol: &'a str,
    size: usize,
    pub string: String,
}

impl GtlIndentLegacy<'_> {
    pub fn new<'a>(symbol: &'a str, size: usize) -> GtlIndentLegacy<'a> {
        let string = symbol.repeat(size);
        GtlIndentLegacy {
            symbol,
            string,
            size,
        }
    }

    pub fn start<'a>(symbol: &'a str) -> GtlIndentLegacy<'a> {
        GtlIndentLegacy::new(symbol, 0)
    }

    pub fn format<T: Into<String>>(&self, code: T) -> String {
        format!("{}{}", self.string, code.into())
    }

    pub fn increment(&self) -> Self {
        Self::new(self.symbol, self.size + 1)
    }
}
