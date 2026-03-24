use crate::prelude::internal::*;

impl<'a> GtlRender<'a> for TSAlias {
    type RenderState = TSRenderState;

    type RenderContext = TSRenderContext<'a>;

    fn render(
        &self,
        state: Self::RenderState,
        context: &mut Self::RenderContext,
    ) -> Result<String> {
        let name = self.name.render(state, context)?;
        let descriptor = self.descriptor.render(state, context)?;

        TSDoc::with_doc(
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
            TSAlias {
                doc: None,
                name: "Name".into(),
                descriptor: TSDescriptor::Primitive(TSPrimitive::String)
            }
            .render(Default::default(), &mut Default::default())
            .unwrap(),
            @"export type Name = string;"
        );
    }

    #[test]
    fn test_render_doc() {
        assert_snapshot!(
            TSAlias {
                doc: Some(TSDoc("Hello, world!".into())),
                name: "Name".into(),
                descriptor: TSDescriptor::Primitive(TSPrimitive::String)
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
