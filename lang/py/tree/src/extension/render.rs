use genotype_config::GTConfig;
use genotype_lang_core_tree::indent::GTIndent;

use crate::PYRender;

use super::PYExtension;

impl PYRender for PYExtension {
    fn render(&self, indent: &GTIndent, config: &GTConfig) -> String {
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
