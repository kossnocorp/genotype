use crate::prelude::internal::*;

pub trait GtlProjectModule<LangConfig: GtlConfig> {
    type Dependency: GtlDependencyIdent;

    fn generate(
        src_path: &GtpSrcDirPath,
        config: &LangConfig,
        module_path: &GtpModulePath,
        module: &GtpModule,
    ) -> Self
    where
        Self: Sized;

    fn dependencies(&self) -> Vec<Self::Dependency>;
}
