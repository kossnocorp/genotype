use genotype_lang_core_tree::{indent::GTIndent, render::GTRender};

use super::RSAny;

impl GTRender for RSAny {
    fn render(&self, _indent: &GTIndent) -> String {
        "Any".to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::indent::rs_indent;

    #[test]
    fn test_render_primitive() {
        assert_eq!(RSAny.render(&rs_indent()), "Any");
    }
}
