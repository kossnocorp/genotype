use crate::prelude::internal::*;
use std::sync::LazyLock;

#[derive(Debug, PartialEq)]
pub struct PyRenderContext<'a> {
    pub config: &'a PyConfigLang,
    pub resolve: GtlRenderResolve<'a, PyRenderState, PyRenderContext<'a>>,
}

impl GtlRenderContext for PyRenderContext<'_> {}

static PY_DEFAULT_CONFIG: LazyLock<PyConfigLang> = LazyLock::new(|| PyConfigLang::default());

impl Default for PyRenderContext<'_> {
    fn default() -> Self {
        Self {
            config: &PY_DEFAULT_CONFIG,
            resolve: GtlRenderResolve::default(),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PyRenderState {
    pub indent: usize,
}

impl GtlRenderState for PyRenderState {
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

impl Default for PyRenderState {
    fn default() -> Self {
        Self { indent: 0 }
    }
}
