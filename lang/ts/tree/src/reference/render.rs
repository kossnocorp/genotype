use genotype_lang_core_tree::{indent::GTIndent, render::GTRender};

use super::TSReference;

impl GTRender for TSReference {
    fn render(&self, indent: &GTIndent) -> String {
        self.0.render(indent)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::indent::ts_indent;

    #[test]
    fn test_render() {
        assert_eq!(TSReference("Foo".into()).render(&ts_indent()), "Foo");
    }
}
