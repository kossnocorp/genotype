use crate::prelude::internal::*;

mod hash;

impl GtpSourceCode {
    pub fn new(content: String) -> Self {
        let hash = GtpSourceCodeHash::new(&content);
        Self { content, hash }
    }
}

impl Display for GtpSourceCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.content)
    }
}

impl Default for GtpSourceCode {
    fn default() -> Self {
        Self::new(String::new())
    }
}

impl AsRef<str> for GtpSourceCode {
    fn as_ref(&self) -> &str {
        &self.content
    }
}
