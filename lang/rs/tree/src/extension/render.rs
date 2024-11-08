use genotype_lang_core_tree::indent::GTIndent;
use genotype_lang_rs_config::RSLangConfig;

use crate::RSRender;

use super::RSExtension;

impl RSRender for RSExtension {
    fn render(&self, indent: &GTIndent, config: &RSLangConfig) -> String {
        self.reference.render(indent, config)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{indent::rs_indent, RSReference};

    #[test]
    fn test_render() {
        assert_eq!(
            RSExtension {
                reference: RSReference::new("Foo".into())
            }
            .render(&rs_indent(), &Default::default()),
            "Foo"
        );
    }
}
