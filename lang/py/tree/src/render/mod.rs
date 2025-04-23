use crate::prelude::internal::*;
use std::sync::LazyLock;

#[derive(Debug, PartialEq)]
pub struct PYRenderContext<'a> {
    pub config: &'a PYLangConfig,
    pub resolve: GtlRenderResolve<'a, PYRenderState, PYRenderContext<'a>>,
}

impl GtlRenderContext for PYRenderContext<'_> {}

static PY_DEFAULT_CONFIG: LazyLock<PYLangConfig> = LazyLock::new(|| PYLangConfig::default());

impl Default for PYRenderContext<'_> {
    fn default() -> Self {
        Self {
            config: &PY_DEFAULT_CONFIG,
            resolve: GtlRenderResolve::default(),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PYRenderState {
    pub indent: usize,
}

impl GtlRenderState for PYRenderState {
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

impl Default for PYRenderState {
    fn default() -> Self {
        Self { indent: 0 }
    }
}
