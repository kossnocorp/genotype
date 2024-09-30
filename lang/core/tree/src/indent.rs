pub struct GTIndent<'a> {
    symbol: &'a str,
    size: usize,
    pub string: String,
}

impl GTIndent<'_> {
    pub fn new<'a>(symbol: &'a str, size: usize) -> GTIndent<'a> {
        let string = symbol.repeat(size);
        GTIndent {
            symbol,
            string,
            size,
        }
    }

    pub fn start<'a>(symbol: &'a str) -> GTIndent<'a> {
        GTIndent::new(symbol, 0)
    }

    pub fn format<T: Into<String>>(&self, code: T) -> String {
        format!("{}{}", self.string, code.into())
    }

    pub fn increment(&self) -> Self {
        Self::new(self.symbol, self.size + 1)
    }
}
