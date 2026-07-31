use crate::prelude::internal::*;
use xxhash_rust::xxh64::xxh64;

impl GtpSourceCodeHash {
    pub fn new(source_code: &str) -> Self {
        Self(format!("{:016x}", xxh64(source_code.as_bytes(), 0)))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl PartialEq<str> for GtpSourceCodeHash {
    fn eq(&self, other: &str) -> bool {
        self.0 == other
    }
}

impl Default for GtpSourceCodeHash {
    fn default() -> Self {
        Self::new("")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hashes_source_code() {
        assert_eq!(GtpSourceCodeHash::new("model").as_str(), "e1eeca7af8e0b529");
    }
}
