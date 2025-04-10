use genotype_lang_core_tree::*;
use genotype_lang_rs_config::RSLangConfig;
use std::sync::LazyLock;

#[derive(Debug, Clone, PartialEq)]
pub struct RSRenderContext<'a> {
    pub indent: usize,
    pub config: &'a RSLangConfig,
}

impl GtlRenderContext for RSRenderContext<'_> {
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

static RS_DEFAULT_CONFIG: LazyLock<RSLangConfig> = LazyLock::new(|| RSLangConfig::default());

impl Default for RSRenderContext<'_> {
    fn default() -> Self {
        Self {
            indent: 0,
            config: &RS_DEFAULT_CONFIG,
        }
    }
}
