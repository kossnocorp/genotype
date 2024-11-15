use genotype_lang_core_tree::indent::GTIndent;
use genotype_lang_rs_config::RSLangConfig;
use miette::Result;

pub trait RSRender {
    fn render(&self, indent: &GTIndent, config: &RSLangConfig) -> Result<String>;
}
