use crate::prelude::internal::*;

impl<Lang: GtlConfig> GtConfigPkg<'_, Lang> {
    pub fn pkg_path(&self) -> GtPkgPath {
        self.dist.join(&self.target.src_relative_path()).into()
    }
}
