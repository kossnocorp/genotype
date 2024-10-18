use genotype_lang_core_tree::{indent::GTIndent, render::GTRender};

use super::PYKey;

impl GTRender for PYKey {
    fn render(&self, _indent: &GTIndent) -> String {
        self.name.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::indent::py_indent;

    #[test]
    fn test_render() {
        assert_eq!(PYKey::new("foo".into(), None).render(&py_indent()), "foo");
    }
}
