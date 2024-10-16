use genotype_lang_core_tree::{indent::GTIndent, render::GTRender};

use super::PYKey;

impl GTRender for PYKey {
    fn render(&self, _indent: &GTIndent) -> String {
        self.0.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::indent::py_indent;

    #[test]
    fn test_render() {
        assert_eq!(PYKey("foo".into()).render(&py_indent()), "foo");
    }
}
