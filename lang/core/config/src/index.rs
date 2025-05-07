use crate::*;

pub trait GtlConfig: Default {
    type PackagePath: GtlConfigPackagePathSetting;

    fn common(&self) -> &GtlConfigCommon<Self::PackagePath>;

    fn src_dir_name<'a>(&'a self) -> &'a str {
        "src"
    }

    fn out_path(&self) -> GtlConfigPackagePath {
        self.common().out.to_path()
    }

    fn src_path(&self) -> GtlConfigPackagePath {
        self.out_path().join(self.src_dir_name().into())
    }
}
