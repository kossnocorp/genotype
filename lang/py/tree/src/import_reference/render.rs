use genotype_lang_core_tree::{indent::GTIndent, render::GTRender};

use super::PYImportReference;

impl GTRender for PYImportReference {
    fn render(&self, indent: &GTIndent) -> String {
        match self {
            PYImportReference::Default(name) => {
                if let Some(name) = name {
                    name.render(indent)
                } else {
                    "".into()
                }
            }

            PYImportReference::Glob => "*".into(),

            PYImportReference::Named(names) => names
                .iter()
                .map(|name| name.render(indent))
                .collect::<Vec<String>>()
                .join(", "),
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
            PYImportReference::Default(Some("Name".into())).render(&py_indent()),
            "Name"
        );
        assert_eq!(PYImportReference::Default(None).render(&py_indent()), "");
    }

    #[test]
    fn test_render_glob() {
        assert_eq!(PYImportReference::Glob.render(&py_indent()), "*");
    }

    #[test]
    fn test_render_named() {
        assert_eq!(
            PYImportReference::Named(vec![
                PYImportName::Name("Name".into()),
                PYImportName::Alias("Name".into(), "Alias".into()),
            ])
            .render(&py_indent()),
            "Name, Name as Alias"
        );
    }
}
