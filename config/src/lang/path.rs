// use crate::prelude::internal::*;
// use delegate::delegate;

// impl GtConfig {
//     pub fn lang_package_path(&self, lang: GtConfigLangIdent, path: PathBuf) -> PathBuf {
//         self.out_path(self.lang_config(lang).package_path(path))
//     }

//     pub fn lang_src_dir_path(&self, lang: GtConfigLangIdent) -> PathBuf {
//         self.out_path(self.lang_config(lang).src_dir_path())
//     }

//     pub fn lang_src_file_path(&self, lang: GtConfigLangIdent, path: PathBuf) -> PathBuf {
//         self.out_path(self.lang_config(lang).src_file_path(path))
//     }
// }

// impl GtConfigLangWrapper<'_> {
//     delegate! {
//       to match self {
//         GtConfigLang::Py(config) => config,
//         GtConfigLang::Rs(config) => config,
//         GtConfigLang::Ts(config) => config,
//       } {
//         pub fn package_path(&self, path: PathBuf) -> PathBuf;
//         pub fn src_dir_path(&self) -> PathBuf;
//         pub fn src_file_path(&self, path: PathBuf) -> PathBuf;
//       }
//     }

//     // pub fn package_path(&self, path: PathBuf) -> PathBuf {
//     //     match self {
//     //         GtConfigLang::Py(config) => config.package_path(path),
//     //         GtConfigLang::Rs(config) => config.package_path(path),
//     //         GtConfigLang::Ts(config) => config.package_path(path),
//     //     }
//     // }

//     // pub fn src_dir_path(&self) -> PathBuf {
//     //     match self {
//     //         GtConfigLang::Py(config) => config.src_dir_path(),
//     //         GtConfigLang::Rs(config) => config.src_dir_path(),
//     //         GtConfigLang::Ts(config) => config.src_dir_path(),
//     //     }
//     // }

//     // pub fn src_file_path(&self, path: PathBuf) -> PathBuf {
//     //     match self {
//     //         GtConfigLang::Py(config) => config.src_file_path(path),
//     //         GtConfigLang::Rs(config) => config.src_file_path(path),
//     //         GtConfigLang::Ts(config) => config.src_file_path(path),
//     //     }
//     // }
// }
