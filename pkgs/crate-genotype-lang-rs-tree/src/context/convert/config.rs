use crate::prelude::internal::*;

impl RsConvertContext {
    pub fn config(&self) -> &RsConfigLang {
        &self.config
    }

    pub fn assign_config(&mut self, config: RsConfigLang) -> &mut Self {
        self.config = config;
        self
    }
}
