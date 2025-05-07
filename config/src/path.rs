use std::path::PathBuf;

use crate::GtConfig;

impl GtConfig {
    pub fn out_path(&self, path: PathBuf) -> PathBuf {
        self.out.join(path)
    }
}
