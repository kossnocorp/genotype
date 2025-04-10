use crate::*;
use genotype_lang_core_tree::*;
use miette::Result;

impl<'a> GtlRender<'a> for RSReference {
    type RenderContext = RSRenderContext<'a>;

    fn render(&self, context: &mut Self::RenderContext) -> Result<String> {
        self.identifier.render(context)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use genotype_parser::*;

    #[test]
    fn test_render() {
        assert_eq!(
            "Foo",
            RSReference {
                id: GTReferenceId("module".into(), (0, 0).into()),
                identifier: "Foo".into(),
                definition_id: GTDefinitionId("module".into(), "Foo".into())
            }
            .render(&mut Default::default())
            .unwrap(),
        );
    }
}
