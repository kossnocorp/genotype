use genotype_lang_core_tree::{indent::GTIndent, render::GTRender};

use super::PYImportReference;

impl GTRender for PYImportReference {
    fn render(&self, indent: &GTIndent) -> String {
        match self {
            PYImportReference::Default(name) => name.clone(),

            PYImportReference::Glob(name) => format!("* as {}", name),

            PYImportReference::Named(names) => {
                let names = names
                    .iter()
                    .map(|name| name.render(indent))
                    .collect::<Vec<String>>()
                    .join(", ");
                format!("{{ {} }}", names)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;

    #[test]
    fn test_render_default() {
        assert_eq!(
            PYImportReference::Default("Name".into()).render(&py_indent()),
            "Name"
        );
    }

    #[test]
    fn test_render_glob() {
        assert_eq!(
            PYImportReference::Glob("name".into()).render(&py_indent()),
            "* as name"
        );
    }

    #[test]
    fn test_render_named() {
        assert_eq!(
            PYImportReference::Named(vec![
                PYImportName::Name("Name".into()),
                PYImportName::Alias("Name".into(), "Alias".into()),
            ])
            .render(&py_indent()),
            "{ Name, Name as Alias }"
        );
    }
}
