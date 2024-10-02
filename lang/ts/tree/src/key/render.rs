use genotype_lang_core_tree::{indent::GTIndent, render::GTRender};

use super::TSKey;

impl GTRender for TSKey {
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
                assert_eq!(TSKey("foo".into()).render(&ts_indent()), "foo");
    }
}
