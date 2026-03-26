use crate::prelude::internal::*;

mod edition;

#[derive(Default, Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct RsConfig {
    #[serde(flatten)]
    pub lang: RsConfigLang,
    #[serde(flatten)]
    pub common: GtlConfigCommon<RsPkgPath>,
}

impl GtlConfig for RsConfig {
    type PkgPath = RsPkgPath;

    fn common(&self) -> &GtlConfigCommon<Self::PkgPath> {
        &self.common
    }
}

impl GtlConfigHealth for RsConfig {
    fn health_check(&self) -> Vec<GtlConfigNotice> {
        let mut notices = vec![];

        if let Some(notice) = self.rust_edition_health_check() {
            notices.push(notice);
        }

        notices
    }
}
