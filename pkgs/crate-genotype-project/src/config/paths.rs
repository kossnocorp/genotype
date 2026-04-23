use crate::prelude::internal::*;

impl GtpConfig {
    /// Relative root path.
    pub fn root_path(&self) -> &RelativePathBuf {
        &self.root
    }

    /// Relative dist path.
    pub fn dist_path(&self) -> &RelativePathBuf {
        &self.dist
    }

    /// Relative src path.
    pub fn src_path(&self) -> &RelativePathBuf {
        &self.src
    }

    /// Relative entry pattern.
    pub fn entry_path(&self) -> &RelativePathBuf {
        &self.entry
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_dist_path() {
        let config = GtpConfig {
            ..Default::default()
        };
        assert_eq!(config.dist_path().as_str(), "dist");
    }
}
