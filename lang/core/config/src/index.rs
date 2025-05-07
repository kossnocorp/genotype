use crate::prelude::internal::*;
use toml::Table;

pub trait GtlConfig: Default {
    type PkgPath: GtlConfigPkgPathSetting;

    fn common(&self) -> &GtlConfigCommon<Self::PkgPath>;

    fn src_dir_name<'a>(&'a self) -> &'a str {
        "src"
    }

    fn src_relative_path(&self) -> GtDistRelativePath {
        self.common().out.to_path()
    }

    fn manifest(&self) -> &Table {
        &self.common().manifest
    }

    // fn dist_relative_path(&self) -> GtDistRelativePath {
    //     self.common().out.to_path()
    // }

    // fn src_path(&self) -> GtDistRelativePath {
    //     self.dist_relative_path().join(self.src_dir_name().into())
    // }
}
