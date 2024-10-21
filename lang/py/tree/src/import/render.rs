use genotype_lang_core_tree::{indent::GTIndent, render::GTRender};

use crate::PYImportReference;

use super::PYImport;

impl GTRender for PYImport {
    fn render(&self, indent: &GTIndent) -> String {
        let path = self.path.render(indent);
        let reference = self.reference.render(indent);

        match self.reference {
            PYImportReference::Default(_) => {
                if reference.is_empty() {
                    format!(r#"import {}"#, path)
                } else {
                    format!(r#"import {} as {}"#, path, reference)
                }
            }

            _ => {
                format!(r#"from {} import {}"#, path, reference)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use super::*;
    use crate::*;

    #[test]
    fn test_render_default() {
        assert_eq!(
            PYImport {
                path: ".path.to.module".into(),
                reference: PYImportReference::Default(Some("name".into())),
                dependency: PYDependency::Local(".path.to.module".into())
            }
            .render(&py_indent()),
            r#"import .path.to.module as name"#
        );
        assert_eq!(
            PYImport {
                path: ".path.to.module".into(),
                reference: PYImportReference::Default(None),
                dependency: PYDependency::Local(".path.to.module".into())
            }
            .render(&py_indent()),
            r#"import .path.to.module"#
        );
    }

    #[test]
    fn test_render_glob() {
        assert_eq!(
            PYImport {
                path: ".path.to.module".into(),
                reference: PYImportReference::Glob,
                dependency: PYDependency::Local(".path.to.module".into())
            }
            .render(&py_indent()),
            r#"from .path.to.module import *"#
        );
    }

    #[test]
    fn test_render_named() {
        assert_eq!(
            PYImport {
                path: ".path.to.module".into(),
                reference: PYImportReference::Named(vec![
                    PYImportName::Name("Name".into()),
                    PYImportName::Alias("Name".into(), "Alias".into()),
                ]),
                dependency: PYDependency::Local(".path.to.module".into())
            }
            .render(&py_indent()),
            r#"from .path.to.module import Name, Name as Alias"#
        );
    }
}
