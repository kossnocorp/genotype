use genotype_lang_core_tree::{indent::GTIndent, render::GTRender};

use super::PYExtension;

impl GTRender for PYExtension {
    fn render(&self, indent: &GTIndent) -> String {
        self.reference.render(indent)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::indent::py_indent;

    #[test]
    fn test_render() {
        assert_eq!(
            PYExtension {
                reference: "Foo".into()
            }
            .render(&py_indent()),
            "Foo"
        );
    }
}
