use genotype_lang_core_tree::indent::GTIndent;
use genotype_lang_py_config::PYLangConfig;

pub trait PYRender {
    fn render(&self, indent: &GTIndent, config: &PYLangConfig) -> String;
}
