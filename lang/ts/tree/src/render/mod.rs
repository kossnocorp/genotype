use crate::prelude::internal::*;
use std::sync::LazyLock;

#[derive(Debug, Clone, PartialEq)]
pub struct TSRenderContext<'a> {
    pub config: &'a TsConfigLang,
}

impl<'a> GtlRenderContext for TSRenderContext<'_> {}

static TS_DEFAULT_CONFIG: LazyLock<TsConfigLang> = LazyLock::new(|| TsConfigLang::default());

impl Default for TSRenderContext<'_> {
    fn default() -> Self {
        Self {
            config: &TS_DEFAULT_CONFIG,
        }
    }
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct TSRenderState {
    indent: usize,
}

impl<'a> GtlRenderState for TSRenderState {
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

impl Default for TSRenderState {
    fn default() -> Self {
        Self { indent: 0 }
    }
}
