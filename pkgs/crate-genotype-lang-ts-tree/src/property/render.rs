use crate::prelude::internal::*;

impl<'a> GtlRender<'a> for TSProperty {
    type RenderState = TSRenderState;

    type RenderContext = TSRenderContext<'a>;

    fn render(
        &self,
        state: Self::RenderState,
        context: &mut Self::RenderContext,
    ) -> Result<String> {
        let name = self.name.render(state, context)?;
        let descriptor = self.descriptor.render(state, context)?;
        let str = format!(
            "{}{name}{}: {descriptor}",
            state.indent_str(),
            if self.required { "" } else { "?" },
        );

        TSDoc::with_doc(&self.doc, state, context, str, false)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use insta::assert_snapshot;

    #[test]
    fn test_render_primitive() {
        assert_snapshot!(
            TSProperty {
                doc: None,
                name: "name".into(),
                descriptor: TSDescriptor::Primitive(TSPrimitive::String),
                required: true
            }
            .render(Default::default(), &mut Default::default())
            .unwrap(),
            @"name: string"
        );
        assert_snapshot!(
            TSProperty {
                doc: None,
                name: "name".into(),
                descriptor: TSDescriptor::Reference("Name".into()),
                required: true
            }
            .render(Default::default(), &mut Default::default())
            .unwrap(),
            @"name: Name"
        );
    }

    #[test]
    fn test_render_indent() {
        assert_snapshot!(
            TSProperty {
                doc: None,
                name: "name".into(),
                descriptor: TSDescriptor::Primitive(TSPrimitive::String),
                required: true
            }
            .render(
                TSRenderState::default().indent_inc(),
                &mut Default::default()
            )
            .unwrap(),
            @"  name: string"
        );
    }

    #[test]
    fn test_render_required() {
        assert_snapshot!(
            TSProperty {
                doc: None,
                name: "name".into(),
                descriptor: TSDescriptor::Primitive(TSPrimitive::String),
                required: false
            }
            .render(Default::default(), &mut Default::default())
            .unwrap(),
            @"name?: string"
        );
    }

    #[test]
    fn test_render_doc() {
        assert_snapshot!(
            TSProperty {
                doc: Some(TSDoc("Hello, world!".into())),
                name: "name".into(),
                descriptor: TSDescriptor::Primitive(TSPrimitive::String),
                required: true
            }
            .render(Default::default(), &mut Default::default())
            .unwrap(),
            @"
        /** Hello, world! */
        name: string
        "
        );
    }
}
