use std::sync::LazyLock;

use genotype_lang_core_tree::*;
use genotype_lang_py_config::PYLangConfig;

#[derive(Debug, Clone, PartialEq)]
pub struct PYRenderContext<'a> {
    pub indent: usize,
    pub config: &'a PYLangConfig,
}

impl GtlRenderContext for PYRenderContext<'_> {
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

static PY_DEFAULT_CONFIG: LazyLock<PYLangConfig> = LazyLock::new(|| PYLangConfig::default());

impl Default for PYRenderContext<'_> {
    fn default() -> Self {
        Self {
            indent: 0,
            config: &PY_DEFAULT_CONFIG,
        }
    }
}
