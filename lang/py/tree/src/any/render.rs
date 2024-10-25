use genotype_lang_core_tree::{indent::GTIndent, render::GTRender};

use super::PYAny;

impl GTRender for PYAny {
    fn render(&self, _indent: &GTIndent) -> String {
        "Any".to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::indent::py_indent;

    #[test]
    fn test_render_primitive() {
        assert_eq!(PYAny.render(&py_indent()), "Any");
    }
}
