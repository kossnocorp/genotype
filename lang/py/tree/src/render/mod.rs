use genotype_lang_core_tree::indent::GTIndent;

use crate::PYOptions;

pub trait PYRender {
    fn render(&self, indent: &GTIndent, options: &PYOptions) -> String;
}
