use genotype_lang_core_tree::*;

#[derive(Debug, Clone, PartialEq)]
pub struct TSRenderContext<'a> {
    pub indent_legacy: GtlIndentLegacy<'a>,
    indent: usize,
}

impl<'a> GtlRenderContext for TSRenderContext<'a> {
    fn indent_inc(&self) -> Self {
        Self {
            indent_legacy: self.indent_legacy.increment(),
            indent: self.indent + 1,
            ..*self
        }
    }

    fn indent_level(&self) -> usize {
        self.indent
    }
}

impl Default for TSRenderContext<'_> {
    fn default() -> Self {
        Self {
            indent_legacy: GtlIndentLegacy::start("  "),
            indent: 0,
        }
    }
}
