use genotype_lang_core_tree::{indent::GTIndent, render::GTRender};

use super::RSIdentifier;

impl GTRender for RSIdentifier {
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
        assert_eq!(RSIdentifier("Foo".into()).render(&rs_indent()), "Foo");
    }
}
