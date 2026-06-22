use crate::prelude::internal::*;
use std::sync::LazyLock;

mod error;
pub use error::*;

mod result;
pub use result::*;

mod types;
pub use types::*;

#[derive(Debug, PartialEq)]
pub struct PyRenderContext<'config> {
    pub config: &'config PyConfigLang,
}

impl<'config> PyRenderContext<'config> {
    pub fn new(config: &'config PyConfigLang) -> Self {
        Self { config }
    }
}

impl GtlRenderContext for PyRenderContext<'_> {}

static PY_DEFAULT_CONFIG: LazyLock<PyConfigLang> = LazyLock::new(PyConfigLang::default);

impl Default for PyRenderContext<'_> {
    fn default() -> Self {
        Self {
            config: &PY_DEFAULT_CONFIG,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub struct PyRenderState {
    pub indent: usize,
}

impl GtlRenderState for PyRenderState {
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
