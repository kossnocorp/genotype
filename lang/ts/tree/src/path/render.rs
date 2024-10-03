use genotype_lang_core_tree::{indent::GTIndent, render::GTRender};

use super::TSPath;

impl GTRender for TSPath {
    fn render(&self, _indent: &GTIndent) -> String {
        self.0.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::indent::ts_indent;

    #[test]
    fn test_render() {
        assert_eq!(
            TSPath("./path/to/module.ts".into()).render(&ts_indent()),
            "./path/to/module.ts"
        );
    }
}
