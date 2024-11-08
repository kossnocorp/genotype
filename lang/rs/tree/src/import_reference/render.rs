use genotype_lang_core_tree::{indent::GTIndent, render::GTRender};

use super::RSImportReference;

impl GTRender for RSImportReference {
    fn render(&self, indent: &GTIndent) -> String {
        match self {
            RSImportReference::Default(name) => {
                if let Some(name) = name {
                    name.render(indent)
                } else {
                    "".into()
                }
            }

            RSImportReference::Glob => "*".into(),

            RSImportReference::Named(names) => names
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
            RSImportReference::Default(Some("Name".into())).render(&rs_indent()),
            "Name"
        );
        assert_eq!(RSImportReference::Default(None).render(&rs_indent()), "");
    }

    #[test]
    fn test_render_glob() {
        assert_eq!(RSImportReference::Glob.render(&rs_indent()), "*");
    }

    #[test]
    fn test_render_named() {
        assert_eq!(
            RSImportReference::Named(vec![
                RSImportName::Name("Name".into()),
                RSImportName::Alias("Name".into(), "Alias".into()),
            ])
            .render(&rs_indent()),
            "Name, Name as Alias"
        );
    }
}
