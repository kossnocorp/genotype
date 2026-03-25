use crate::prelude::internal::*;
use std::sync::LazyLock;

#[derive(Debug, Clone, PartialEq)]
pub struct TsRenderContext<'a> {
    pub config: &'a TsConfigLang,
}

impl<'a> GtlRenderContext for TsRenderContext<'_> {}

static TS_DEFAULT_CONFIG: LazyLock<TsConfigLang> = LazyLock::new(|| TsConfigLang::default());

impl Default for TsRenderContext<'_> {
    fn default() -> Self {
        Self {
            config: &TS_DEFAULT_CONFIG,
        }
    }
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct TsRenderState {
    indent: usize,
}

impl<'a> GtlRenderState for TsRenderState {
    fn indent_inc(&self) -> Self {
        Self {
            indent: self.indent + 1,
            ..*self
        }
    }

    fn indent_level(&self) -> usize {
        self.indent
    }
}

impl Default for TsRenderState {
    fn default() -> Self {
        Self { indent: 0 }
    }
}
