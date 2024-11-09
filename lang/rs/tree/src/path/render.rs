use genotype_lang_core_tree::{indent::GTIndent, render::GTRender};

use super::RSPath;

impl GTRender for RSPath {
    fn render(&self, _indent: &GTIndent) -> String {
        self.0.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::indent::rs_indent;

    #[test]
    fn test_render() {
        assert_eq!(
            RSPath("self::path::to::module".into()).render(&rs_indent()),
            "self::path::to::module"
        );
    }
}
