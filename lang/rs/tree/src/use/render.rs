use crate::*;
use genotype_lang_core_tree::*;
use miette::Result;

impl<'a> GtlRender<'a> for RSUse {
    type RenderState = RSRenderState;

    type RenderContext = RSRenderContext<'a>;

    fn render(&self, state: Self::RenderState, context: &mut Self::RenderContext) -> Result<String> {
        let path = self.dependency.as_path();
        let reference = self.reference.render(state, context)?;

        Ok(match self.reference {
            RSUseReference::Module => format!(r#"use {path};"#),
            _ => format!(r#"use {path}::{reference};"#),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use genotype_parser::GTModuleId;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_render_module() {
        assert_eq!(
            RSUse {
                reference: RSUseReference::Module,
                dependency: RSDependency::Local(RSPath(
                    GTModuleId("path/to/module".into()),
                    "self::path::to::module".into()
                ))
            }
            .render(Default::default(), &mut Default::default())
            .unwrap(),
            r#"use self::path::to::module;"#
        );
    }

    #[test]
    fn test_render_glob() {
        assert_eq!(
            RSUse {
                reference: RSUseReference::Glob,
                dependency: RSDependency::Local(RSPath(
                    GTModuleId("path/to/module".into()),
                    "self::path::to::module".into()
                ))
            }
            .render(Default::default(), &mut Default::default())
            .unwrap(),
            r#"use self::path::to::module::*;"#
        );
    }

    #[test]
    fn test_render_named() {
        assert_eq!(
            RSUse {
                reference: RSUseReference::Named(vec![
                    RSUseName::Name("Name".into()),
                    RSUseName::Alias("Name".into(), "Alias".into()),
                ]),
                dependency: RSDependency::Local(RSPath(
                    GTModuleId("path/to/module".into()),
                    "self::path::to::module".into()
                ))
            }
            .render(Default::default(), &mut Default::default())
            .unwrap(),
            r#"use self::path::to::module::{Name, Name as Alias};"#
        );
    }
}
