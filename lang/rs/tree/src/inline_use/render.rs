use genotype_lang_core_tree::{indent::GTIndent, render::GTRender};

use super::RSInlineUse;

impl GTRender for RSInlineUse {
    fn render(&self, indent: &GTIndent) -> String {
        format!("{}::{}", self.path.render(indent), self.name.render(indent))
    }
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use super::*;
    use crate::*;

    #[test]
    fn test_render() {
        assert_eq!(
            RSInlineUse {
                path: "self::path::to::module".into(),
                name: "Name".into(),
            }
            .render(&rs_indent()),
            "self::path::to::module::Name"
        );
    }
}
