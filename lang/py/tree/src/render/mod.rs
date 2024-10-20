use genotype_config::GTConfig;
use genotype_lang_core_tree::indent::GTIndent;

pub trait PYRender {
    fn render(&self, indent: &GTIndent, config: &GTConfig) -> String;
}
