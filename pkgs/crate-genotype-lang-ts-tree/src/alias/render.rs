use crate::prelude::internal::*;

impl<'a> GtlRender<'a> for TsAlias {
    type RenderState = TsRenderState;

    type RenderContext = TsRenderContext<'a>;

    fn render(
        &self,
        state: Self::RenderState,
        context: &mut Self::RenderContext,
    ) -> Result<String> {
        let name = self.name.render(state, context)?;
        let descriptor = self.descriptor.render(state, context)?;

        TsDoc::with_doc(
            &self.doc,
            state,
            context,
            format!("export type {name} = {descriptor};",),
            false,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use insta::assert_snapshot;

    #[test]
    fn test_render() {
        assert_snapshot!(
            TsAlias {
                doc: None,
                name: "Name".into(),
                descriptor: TsDescriptor::Primitive(TsPrimitive::String)
            }
            .render(Default::default(), &mut Default::default())
            .unwrap(),
            @"export type Name = string;"
        );
    }

    #[test]
    fn test_render_doc() {
        assert_snapshot!(
            TsAlias {
                doc: Some(TsDoc("Hello, world!".into())),
                name: "Name".into(),
                descriptor: TsDescriptor::Primitive(TsPrimitive::String)
            }
            .render(Default::default(), &mut Default::default())
            .unwrap(),
            @"
        /** Hello, world! */
        export type Name = string;
        "
        );
    }
}
