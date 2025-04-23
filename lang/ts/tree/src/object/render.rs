use crate::prelude::internal::*;

impl<'a> GtlRender<'a> for TSObject {
    type RenderState = TSRenderState;

    type RenderContext = TSRenderContext;

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
    use pretty_assertions::assert_eq;

    #[test]
    fn test_render_empty() {
        assert_eq!(
            TSObject { properties: vec![] }
                .render(Default::default(), &mut Default::default())
                .unwrap(),
            "{\n}"
        );
    }

    #[test]
    fn test_render_properties() {
        assert_eq!(
            TSObject {
                properties: vec![
                    TSProperty {
                        doc: None,
                        name: "name".into(),
                        descriptor: TSDescriptor::Primitive(TSPrimitive::String),
                        required: true
                    },
                    TSProperty {
                        doc: None,
                        name: "age".into(),
                        descriptor: TSDescriptor::Primitive(TSPrimitive::Number),
                        required: false
                    }
                ]
            }
            .render(Default::default(), &mut Default::default())
            .unwrap(),
            "{\n  name: string,\n  age?: number\n}"
        );
    }

    #[test]
    fn test_render_indent() {
        assert_eq!(
            TSObject {
                properties: vec![
                    TSProperty {
                        doc: None,
                        name: "name".into(),
                        descriptor: TSDescriptor::Primitive(TSPrimitive::String),
                        required: true
                    },
                    TSProperty {
                        doc: None,
                        name: "age".into(),
                        descriptor: TSDescriptor::Primitive(TSPrimitive::Number),
                        required: false
                    }
                ]
            }
            .render(
                TSRenderState::default().indent_inc(),
                &mut Default::default()
            )
            .unwrap(),
            "{\n    name: string,\n    age?: number\n  }"
        );
    }
}
