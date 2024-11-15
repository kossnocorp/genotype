use genotype_lang_core_tree::indent::GTIndent;
use genotype_lang_rs_config::RSLangConfig;
use miette::Result;

use crate::{RSImportReference, RSRender};

use super::RSImport;

impl RSRender for RSImport {
    fn render(&self, indent: &GTIndent, config: &RSLangConfig) -> Result<String> {
        let path = self.path.render(indent, config)?;
        let reference = self.reference.render(indent, config)?;

        Ok(match self.reference {
            RSImportReference::Module => format!(r#"use {path};"#),
            _ => format!(r#"use {path}::{reference};"#),
        })
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
            .render(&rs_indent(), &Default::default())
            .unwrap(),
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
            .render(&rs_indent(), &Default::default())
            .unwrap(),
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
            .render(&rs_indent(), &Default::default())
            .unwrap(),
            r#"use self::path::to::module::{Name, Name as Alias};"#
        );
    }
}
