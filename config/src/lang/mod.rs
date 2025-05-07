use crate::prelude::internal::*;

mod path;

impl GtConfig {
    pub fn lang_config<'a>(&'a self, lang: GtConfigLangIdent) -> GtConfigLangWrapper<'a> {
        match lang {
            GtConfigLangIdent::Py => GtConfigLangWrapper::Py(&self.py),
            GtConfigLangIdent::Rs => GtConfigLangWrapper::Rs(&self.rs),
            GtConfigLangIdent::Ts => GtConfigLangWrapper::Ts(&self.ts),
        }
    }
}

pub enum GtConfigLangWrapper<'a> {
    Py(&'a PyConfig),
    Rs(&'a RsConfig),
    Ts(&'a TsConfig),
}

pub enum GtConfigLangIdent {
    Py,
    Rs,
    Ts,
}

pub struct GtConfigLang<'a, Lang: GtlConfig> {
    /// Dist directory relative to the working directory.
    dist: &'a GtCwdPath,
    /// Language config.
    lang: Lang,
}
