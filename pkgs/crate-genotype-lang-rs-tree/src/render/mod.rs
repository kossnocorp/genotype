use crate::prelude::internal::*;
use std::sync::LazyLock;

mod error;
pub use error::*;

mod result;
pub use result::*;

mod types;
pub use types::*;

#[derive(Debug, Clone, PartialEq)]
pub struct RsRenderContext<'config> {
    pub config: &'config RsConfigLang,
}

impl GtlRenderContext for RsRenderContext<'_> {}

impl<'config> RsRenderContext<'config> {
    pub fn new(config: &'config RsConfigLang) -> Self {
        Self { config }
    }
}

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
        }
    }

    fn indent_level(&self) -> usize {
        self.indent
    }
}
