use crate::*;
use delegate::delegate;
use std::path::PathBuf;

// /// Target package path. It is relative to the out path, e.g. "src/index.ts".
// pub struct GtlConfigPackagePath(PathBuf);

// impl GtlConfigPackagePath {
//     pub fn new(path: PathBuf) -> Self {
//         GtlConfigPackagePath(path)
//     }

//     pub fn join(&self, path: PathBuf) -> Self {
//         GtlConfigPackagePath(self.0.join(path))
//     }
// }
