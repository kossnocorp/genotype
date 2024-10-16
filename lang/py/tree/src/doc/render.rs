use genotype_lang_core_tree::{indent::GTIndent, render::GTRender};

use super::PYDoc;

impl GTRender for PYDoc {
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
        assert_eq!(
            PYDoc("Hello, world!".into()).render(&py_indent()),
            "Hello, world!"
        );
    }
}
