use genotype_lang_core_tree::{indent::GTIndent, render::GTRender};

use crate::RSImportReference;

use super::RSImport;

impl GTRender for RSImport {
    fn render(&self, indent: &GTIndent) -> String {
        let path = self.path.render(indent);
        let reference = self.reference.render(indent);

        match self.reference {
            RSImportReference::Module => format!(r#"use {};"#, path),
            _ => format!(r#"use {}::{};"#, path, reference),
        }
    }
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use super::*;
    use crate::*;

    #[test]
    fn test_render_module() {
        assert_eq!(
            RSImport {
                path: "self::path::to::module".into(),
                reference: RSImportReference::Module,
                dependency: RSDependency::Local("self::path::to::module".into())
            }
            .render(&rs_indent()),
            r#"use self::path::to::module;"#
        );
    }

    #[test]
    fn test_render_glob() {
        assert_eq!(
            RSImport {
                path: "self::path::to::module".into(),
                reference: RSImportReference::Glob,
                dependency: RSDependency::Local("self::path::to::module".into())
            }
            .render(&rs_indent()),
            r#"use self::path::to::module::*;"#
        );
    }

    #[test]
    fn test_render_named() {
        assert_eq!(
            RSImport {
                path: "self::path::to::module".into(),
                reference: RSImportReference::Named(vec![
                    RSImportName::Name("Name".into()),
                    RSImportName::Alias("Name".into(), "Alias".into()),
                ]),
                dependency: RSDependency::Local("self::path::to::module".into())
            }
            .render(&rs_indent()),
            r#"use self::path::to::module::{Name, Name as Alias};"#
        );
    }
}
