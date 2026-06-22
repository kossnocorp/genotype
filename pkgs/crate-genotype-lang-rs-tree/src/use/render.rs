use crate::prelude::internal::*;

impl<'context> GtlRender<'context, RsRenderTypes> for RsUse {
    fn render(
        &self,
        state: RsRenderState,
        context: &mut RsRenderContext,
    ) -> RsRenderResult<String> {
        let path = self.dependency.as_path();
        let reference = self.reference.render(state, context)?;

        todo!();
        // Ok(match self.reference {
        //     RsUseReference::Module => format!(r#"use {path};"#),
        //     _ => format!(r#"use {path}::{reference};"#),
        // })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use genotype_parser::GtModuleId;
    use insta::assert_snapshot;

    #[test]
    fn test_render_module() {
        assert_snapshot!(
            RsUse {
                reference: RsUseReference::Module,
                dependency: RsDependencyIdent::Local(RsPath(
                    GtModuleId("path/to/module".into()),
                    "self::path::to::module".into()
                ))
            }
            .render(Default::default(), &mut Default::default())
            .unwrap(),
            @"use self::path::to::module;"
        );
    }

    #[test]
    fn test_render_glob() {
        assert_snapshot!(
            RsUse {
                reference: RsUseReference::Glob,
                dependency: RsDependencyIdent::Local(RsPath(
                    GtModuleId("path/to/module".into()),
                    "self::path::to::module".into()
                ))
            }
            .render(Default::default(), &mut Default::default())
            .unwrap(),
            @"use self::path::to::module::*;"
        );
    }

    #[test]
    fn test_render_named() {
        assert_snapshot!(
            RsUse {
                reference: RsUseReference::Named(vec![
                    RsUseName::Name("Name".into()),
                    RsUseName::Alias("Name".into(), "Alias".into()),
                ]),
                dependency: RsDependencyIdent::Local(RsPath(
                    GtModuleId("path/to/module".into()),
                    "self::path::to::module".into()
                ))
            }
            .render(Default::default(), &mut Default::default())
            .unwrap(),
            @"use self::path::to::module::{Name, Name as Alias};"
        );
    }
}
