use std::path::PathBuf;

mod common;
pub use common::*;

mod out;
pub use out::*;

pub trait GtlConfig: Default {
    type Out: GtlConfigOut;

    fn common(&self) -> &GtlConfigCommon<Self::Out>;

    fn src_dir_name<'a>(&'a self) -> &'a str {
        "src"
    }

    fn package_path(&self, path: PathBuf) -> PathBuf {
        self.common().out.as_path().join(path)
    }

    fn src_dir_path(&self) -> PathBuf {
        self.package_path(PathBuf::from(self.src_dir_name()))
    }

    fn src_file_path(&self, path: PathBuf) -> PathBuf {
        self.src_dir_path().join(path)
    }
}
