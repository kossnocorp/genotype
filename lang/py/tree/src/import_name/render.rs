use genotype_lang_core_tree::{indent::GTIndent, render::GTRender};

use super::PYImportName;

impl GTRender for PYImportName {
    fn render(&self, indent: &GTIndent) -> String {
        match self {
            PYImportName::Name(name) => name.render(indent),
            PYImportName::Alias(name, alias) => {
                format!("{} as {}", name.render(indent), alias.render(indent))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::indent::py_indent;

    #[test]
    fn test_render_name() {
        assert_eq!(
            PYImportName::Name("Name".into()).render(&py_indent()),
            "Name"
        );
    }

    #[test]
    fn test_render_alias() {
        assert_eq!(
            PYImportName::Alias("Name".into(), "Alias".into()).render(&py_indent()),
            "Name as Alias"
        );
    }
}
