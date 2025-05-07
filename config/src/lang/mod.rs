use delegate::delegate;

use crate::prelude::internal::*;

mod path;

impl GtConfig {
    pub fn lang_config<'a>(&'a self, lang: GtConfigLangIdent) -> GtConfigLang<'a> {
        match lang {
            GtConfigLangIdent::Py => {
                GtConfigLang::new(self.dist_path(), GtConfigLangWrapper::Py(&self.py))
            }
            GtConfigLangIdent::Rs => {
                GtConfigLang::new(self.dist_path(), GtConfigLangWrapper::Rs(&self.rs))
            }
            GtConfigLangIdent::Ts => {
                GtConfigLang::new(self.dist_path(), GtConfigLangWrapper::Ts(&self.ts))
            }
        }
    }
}

pub struct GtConfigLang<'a> {
    /// Dist directory relative to the working directory.
    pub dist: GtDistPath,
    /// Language config.
    pub lang: GtConfigLangWrapper<'a>,
}

impl<'a> GtConfigLang<'a> {
    pub fn new(dist: GtDistPath, lang: GtConfigLangWrapper<'a>) -> Self {
        Self { dist, lang }
    }
}

pub enum GtConfigLangIdent {
    Py,
    Rs,
    Ts,
}

pub enum GtConfigLangWrapper<'a> {
    Py(&'a PyConfig),
    Rs(&'a RsConfig),
    Ts(&'a TsConfig),
}

impl GtConfigLangWrapper<'_> {
    delegate! {
        to match self {
            GtConfigLangWrapper::Py(config) => config,
            GtConfigLangWrapper::Rs(config) => config,
            GtConfigLangWrapper::Ts(config) => config,
        } {
            pub fn common(&self) -> &GtlConfigCommon<GtlConfig::PackagePath>;
        }
    }
}
