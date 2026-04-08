use crate::prelude::internal::*;
use std::sync::LazyLock;

#[derive(Debug, Clone, PartialEq)]
pub struct RsRenderContext<'a> {
    pub config: &'a RsConfigLang,
}

impl GtlRenderContext for RsRenderContext<'_> {}

static RS_DEFAULT_CONFIG: LazyLock<RsConfigLang> = LazyLock::new(RsConfigLang::default);

impl Default for RsRenderContext<'_> {
    fn default() -> Self {
        Self {
            config: &RS_DEFAULT_CONFIG,
        }
    }
}

#[derive(Clone, Copy, PartialEq, Debug, Default)]
pub struct RsRenderState {
    pub indent: usize,
}

impl GtlRenderState for RsRenderState {
    const INDENT: &'static str = "    ";

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
