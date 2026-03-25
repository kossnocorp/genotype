use crate::prelude::internal::*;

impl<'a> GtlRender<'a> for TsObject {
    type RenderState = TsRenderState;

    type RenderContext = TsRenderContext<'a>;

    fn render(
        &self,
        state: Self::RenderState,
        context: &mut Self::RenderContext,
    ) -> Result<String> {
        let properties = self
            .properties
            .iter()
            .map(|property| property.render(state.indent_inc(), context))
            .collect::<Result<Vec<_>>>()?
            .join(",\n");

        Ok(format!(
            "{{\n{properties}{}{}",
            if properties.len() > 0 { "\n" } else { "" },
            state.indent_format("}")
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use insta::assert_snapshot;

    #[test]
    fn test_render_empty() {
        assert_snapshot!(
            TsObject { properties: vec![] }
                .render(Default::default(), &mut Default::default())
                .unwrap(),
            @"
        {
        }
        "
        );
    }

    #[test]
    fn test_render_properties() {
        assert_snapshot!(
            TsObject {
                properties: vec![
                    TsProperty {
                        doc: None,
                        name: "name".into(),
                        descriptor: TsDescriptor::Primitive(TsPrimitive::String),
                        required: true
                    },
                    TsProperty {
                        doc: None,
                        name: "age".into(),
                        descriptor: TsDescriptor::Primitive(TsPrimitive::Number),
                        required: false
                    }
                ]
            }
            .render(Default::default(), &mut Default::default())
            .unwrap(),
            @"
        {
          name: string,
          age?: number
        }
        "
        );
    }

    #[test]
    fn test_render_indent() {
        assert_snapshot!(
            TsObject {
                properties: vec![
                    TsProperty {
                        doc: None,
                        name: "name".into(),
                        descriptor: TsDescriptor::Primitive(TsPrimitive::String),
                        required: true
                    },
                    TsProperty {
                        doc: None,
                        name: "age".into(),
                        descriptor: TsDescriptor::Primitive(TsPrimitive::Number),
                        required: false
                    }
                ]
            }
            .render(
                TsRenderState::default().indent_inc(),
                &mut Default::default()
            )
            .unwrap(),
            @"
        {
            name: string,
            age?: number
          }
        "
        );
    }
}
