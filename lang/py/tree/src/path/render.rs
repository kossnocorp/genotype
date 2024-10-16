use genotype_lang_core_tree::{indent::GTIndent, render::GTRender};

use super::PYPath;

impl GTRender for PYPath {
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
            PYPath(".path.to.module".into()).render(&py_indent()),
            ".path.to.module"
        );
    }
}
