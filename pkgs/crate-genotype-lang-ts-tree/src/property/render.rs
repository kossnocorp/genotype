use crate::prelude::internal::*;

impl<'a> GtlRender<'a> for TsProperty {
    type RenderState = TsRenderState;

    type RenderContext = TsRenderContext<'a>;

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

        TsDoc::with_doc(&self.doc, state, context, str, false)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use insta::assert_snapshot;

    #[test]
    fn test_render_primitive() {
        assert_snapshot!(
            TsProperty {
                doc: None,
                name: "name".into(),
                descriptor: TsDescriptor::Primitive(TsPrimitive::String),
                required: true
            }
            .render(Default::default(), &mut Default::default())
            .unwrap(),
            @"name: string"
        );
        assert_snapshot!(
            TsProperty {
                doc: None,
                name: "name".into(),
                descriptor: TsDescriptor::Reference("Name".into()),
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
            TsProperty {
                doc: None,
                name: "name".into(),
                descriptor: TsDescriptor::Primitive(TsPrimitive::String),
                required: true
            }
            .render(
                TsRenderState::default().indent_inc(),
                &mut Default::default()
            )
            .unwrap(),
            @"  name: string"
        );
    }

    #[test]
    fn test_render_required() {
        assert_snapshot!(
            TsProperty {
                doc: None,
                name: "name".into(),
                descriptor: TsDescriptor::Primitive(TsPrimitive::String),
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
            TsProperty {
                doc: Some(TsDoc("Hello, world!".into())),
                name: "name".into(),
                descriptor: TsDescriptor::Primitive(TsPrimitive::String),
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
