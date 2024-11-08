use genotype_lang_core_tree::indent::GTIndent;
use genotype_lang_rs_config::RSLangConfig;

pub trait RSRender {
    fn render(&self, indent: &GTIndent, config: &RSLangConfig) -> String;
}
