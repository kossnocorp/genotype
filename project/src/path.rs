use crate::prelude::internal::*;
use delegate::delegate;

impl GtProject {
    delegate! {
      to self.config {
        pub fn lang_package_path(&self, lang: GtConfigLangIdent, path: PathBuf) -> PathBuf;
        pub fn lang_src_dir_path(&self, lang: GtConfigLangIdent) -> PathBuf;
        pub fn lang_src_file_path(&self, lang: GtConfigLangIdent, path: PathBuf) -> PathBuf;
      }
    }
}
