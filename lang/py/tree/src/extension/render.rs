use genotype_lang_core_tree::indent::GTIndent;

use crate::{PYOptions, PYRender};

use super::PYExtension;

impl PYRender for PYExtension {
    fn render(&self, indent: &GTIndent, options: &PYOptions) -> String {
        self.reference.render(indent, options)
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
            .render(&py_indent(), &PYOptions::default()),
            "Foo"
        );
    }
}
