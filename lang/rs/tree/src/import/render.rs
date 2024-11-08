use genotype_lang_core_tree::{indent::GTIndent, render::GTRender};

use crate::RSImportReference;

use super::RSImport;

impl GTRender for RSImport {
    fn render(&self, indent: &GTIndent) -> String {
        let path = self.path.render(indent);
        let reference = self.reference.render(indent);

        match self.reference {
            RSImportReference::Default(_) => {
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
            RSImport {
                path: ".path.to.module".into(),
                reference: RSImportReference::Default(Some("name".into())),
                dependency: RSDependency::Local(".path.to.module".into())
            }
            .render(&rs_indent()),
            r#"import .path.to.module as name"#
        );
        assert_eq!(
            RSImport {
                path: ".path.to.module".into(),
                reference: RSImportReference::Default(None),
                dependency: RSDependency::Local(".path.to.module".into())
            }
            .render(&rs_indent()),
            r#"import .path.to.module"#
        );
    }

    #[test]
    fn test_render_glob() {
        assert_eq!(
            RSImport {
                path: ".path.to.module".into(),
                reference: RSImportReference::Glob,
                dependency: RSDependency::Local(".path.to.module".into())
            }
            .render(&rs_indent()),
            r#"from .path.to.module import *"#
        );
    }

    #[test]
    fn test_render_named() {
        assert_eq!(
            RSImport {
                path: ".path.to.module".into(),
                reference: RSImportReference::Named(vec![
                    RSImportName::Name("Name".into()),
                    RSImportName::Alias("Name".into(), "Alias".into()),
                ]),
                dependency: RSDependency::Local(".path.to.module".into())
            }
            .render(&rs_indent()),
            r#"from .path.to.module import Name, Name as Alias"#
        );
    }
}
