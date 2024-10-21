use genotype_lang_py_config::PYLangConfig;
use genotype_lang_core_tree::indent::GTIndent;

pub trait PYRender {
    fn render(&self, indent: &GTIndent, config: &PYLangConfig) -> String;
}
