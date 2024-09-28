pub struct Indent<'a> {
    symbol: &'a str,
    indent: String,
    size: usize,
}

impl Indent<'_> {
    pub fn new<'a>(symbol: &'a str, size: usize) -> Indent<'a> {
        let indent = symbol.repeat(size);
        Indent {
            symbol,
            indent,
            size,
        }
    }

    pub fn start<'a>(symbol: &'a str) -> Indent<'a> {
        Indent::new(symbol, 0)
    }

    pub fn format<T: Into<String>>(&self, code: T) -> String {
        format!("{}{}", self.indent, code.into())
    }

    pub fn increment(&self) -> Self {
        Self::new(self.symbol, self.size + 1)
    }
}
