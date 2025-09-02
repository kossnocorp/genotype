use crate::prelude::internal::*;

#[derive(Default, Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct TsConfig {
    #[serde(flatten)]
    pub common: GtlConfigCommon<TsPkgPath>,

        #[serde(flatten)]
    pub lang: TsConfigLang,
}

impl GtlConfig for TsConfig {
    type PkgPath = TsPkgPath;

    fn common(&self) -> &GtlConfigCommon<Self::PkgPath> {
        &self.common
    }
}
