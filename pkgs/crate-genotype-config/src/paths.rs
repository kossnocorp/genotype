use crate::prelude::internal::*;

impl GtConfig {
    pub fn dist_path(&self) -> GtDistPath {
        self.root.join(&self.out).into()
    }

    pub fn src_path(&self) -> GtSrcPath {
        self.root.join(&self.src).into()
    }

    pub fn entry_path(&self) -> GtEntryPath {
        self.entry.with_parent(self.src_path().relative_path())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_dist_path() {
        let config = GtConfig {
            ..Default::default()
        };
        assert_eq!(config.dist_path().as_str(), "dist");
    }
}
