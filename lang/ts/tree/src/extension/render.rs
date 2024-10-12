use genotype_lang_core_tree::{indent::GTIndent, render::GTRender};

use super::TSExtension;

impl GTRender for TSExtension {
    fn render(&self, indent: &GTIndent) -> String {
        self.reference.render(indent)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::indent::ts_indent;

    #[test]
    fn test_render() {
        assert_eq!(
            TSExtension {
                reference: "Foo".into()
            }
            .render(&ts_indent()),
            "Foo"
        );
    }
}
