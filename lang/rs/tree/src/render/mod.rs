use genotype_lang_core_tree::*;
use genotype_lang_rs_config::RSLangConfig;
use std::sync::LazyLock;

#[derive(Debug, Clone, PartialEq)]
pub struct RSRenderContext<'a> {
    pub config: &'a RSLangConfig,
}

impl GtlRenderContext for RSRenderContext<'_> {}

static RS_DEFAULT_CONFIG: LazyLock<RSLangConfig> = LazyLock::new(|| RSLangConfig::default());

impl Default for RSRenderContext<'_> {
    fn default() -> Self {
        Self {
            config: &RS_DEFAULT_CONFIG,
        }
    }
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct RSRenderState {
    pub indent: usize,
}

impl GtlRenderState for RSRenderState {
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

impl Default for RSRenderState {
    fn default() -> Self {
        Self { indent: 0 }
    }
}
