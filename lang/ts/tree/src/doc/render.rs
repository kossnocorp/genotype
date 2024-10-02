use genotype_lang_core_tree::{indent::GTIndent, render::GTRender};

use super::TSDoc;

impl GTRender for TSDoc {
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
            TSDoc("Hello, world!".into()).render(&ts_indent()),
            "Hello, world!"
        );
    }
}
