use genotype_lang_core_tree::{indent::GTIndent, render::GTRender};

use super::PYReference;

impl GTRender for PYReference {
    fn render(&self, indent: &GTIndent) -> String {
        self.0.render(indent)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::indent::py_indent;

    #[test]
    fn test_render() {
        assert_eq!(PYReference("Foo".into()).render(&py_indent()), "Foo");
    }
}
