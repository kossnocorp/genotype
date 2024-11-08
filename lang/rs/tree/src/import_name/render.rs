use genotype_lang_core_tree::{indent::GTIndent, render::GTRender};

use super::RSImportName;

impl GTRender for RSImportName {
    fn render(&self, indent: &GTIndent) -> String {
        match self {
            RSImportName::Name(name) => name.render(indent),
            RSImportName::Alias(name, alias) => {
                format!("{} as {}", name.render(indent), alias.render(indent))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::indent::rs_indent;

    #[test]
    fn test_render_name() {
        assert_eq!(
            RSImportName::Name("Name".into()).render(&rs_indent()),
            "Name"
        );
    }

    #[test]
    fn test_render_alias() {
        assert_eq!(
            RSImportName::Alias("Name".into(), "Alias".into()).render(&rs_indent()),
            "Name as Alias"
        );
    }
}
