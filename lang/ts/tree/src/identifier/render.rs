use genotype_lang_core_tree::{indent::GTIndent, render::GTRender};

use super::TSIdentifier;

impl GTRender for TSIdentifier {
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
        assert_eq!(TSIdentifier("foo".into()).render(&ts_indent()), "foo");
    }
}
