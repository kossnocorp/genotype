use genotype_lang_core_tree::{indent::GTIndent, render::GTRender};

use super::TSAny;

impl GTRender for TSAny {
    fn render(&self, _indent: &GTIndent) -> String {
        "any".to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::indent::ts_indent;

    #[test]
    fn test_render() {
        assert_eq!(TSAny.render(&ts_indent()), "any");
    }
}
