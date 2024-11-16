use genotype_lang_core_tree::indent::GTIndent;
use genotype_lang_rs_config::RSLangConfig;
use miette::Result;

use crate::{RSRender, RSUseReference};

use super::RSUse;

impl RSRender for RSUse {
    fn render(&self, indent: &GTIndent, config: &RSLangConfig) -> Result<String> {
        let path = self.path.render(indent, config)?;
        let reference = self.reference.render(indent, config)?;

        Ok(match self.reference {
            RSUseReference::Module => format!(r#"use {path};"#),
            _ => format!(r#"use {path}::{reference};"#),
        })
    }
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use crate::*;

    #[test]
    fn test_render_module() {
        assert_eq!(
            RSUse {
                path: "self::path::to::module".into(),
                reference: RSUseReference::Module,
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
            RSUse {
                path: "self::path::to::module".into(),
                reference: RSUseReference::Glob,
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
            RSUse {
                path: "self::path::to::module".into(),
                reference: RSUseReference::Named(vec![
                    RSUseName::Name("Name".into()),
                    RSUseName::Alias("Name".into(), "Alias".into()),
                ]),
                dependency: RSDependency::Local("self::path::to::module".into())
            }
            .render(&rs_indent(), &Default::default())
            .unwrap(),
            r#"use self::path::to::module::{Name, Name as Alias};"#
        );
    }
}
