use genotype_lang_core_tree::indent::GTIndent;
use genotype_lang_py_config::PYLangConfig;

use crate::PYRender;

use super::PYExtension;

impl PYRender for PYExtension {
    fn render(&self, indent: &GTIndent, config: &PYLangConfig) -> String {
        self.reference.render(indent, config)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{indent::py_indent, PYReference};

    #[test]
    fn test_render() {
        assert_eq!(
            PYExtension {
                reference: PYReference::new("Foo".into(), false)
            }
            .render(&py_indent(), &Default::default()),
            "Foo"
        );
    }
}
