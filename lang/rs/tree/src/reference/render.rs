use genotype_lang_core_tree::{indent::GTIndent, render::GTRender};
use genotype_lang_rs_config::RSLangConfig;
use genotype_lang_rs_config::RSVersion;

use crate::RSRender;

use super::RSReference;

impl RSRender for RSReference {
    fn render(&self, indent: &GTIndent, config: &RSLangConfig) -> String {
        self.identifier.render(indent)
    }
}

#[cfg(test)]
mod tests {
    use genotype_lang_rs_config::RSLangConfig;

    use super::*;
    use crate::indent::rs_indent;

    #[test]
    fn test_render() {
        assert_eq!(
            "Foo",
            RSReference::new("Foo".into()).render(&rs_indent(), &Default::default()),
        );
    }
}
