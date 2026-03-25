use crate::prelude::internal::*;

impl<'a> GtlRender<'a> for TsInterface {
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
            .iter()
            .map(|property| format!("{property};"))
            .collect::<Vec<_>>()
            .join("\n");

        let extensions = self
            .extensions
            .iter()
            .map(|extension| extension.render(state, context))
            .collect::<Result<Vec<_>>>()?
            .join(", ");

        let name = self.name.render(state, context)?;
        let extends = if extensions.len() > 0 {
            format!(" extends {}", extensions)
        } else {
            "".into()
        };

        TsDoc::with_doc(
            &self.doc,
            state,
            context,
            format!(
                "{}export interface {name}{extends} {{\n{properties}{}{}",
                state.indent_str(),
                if properties.len() > 0 { "\n" } else { "" },
                state.indent_format("}")
            ),
            false,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use insta::assert_snapshot;

    #[test]
    fn test_render_empty() {
        assert_snapshot!(
            TsInterface {
                doc: None,
                name: "Name".into(),
                extensions: vec![],
                properties: vec![]
            }
            .render(Default::default(), &mut Default::default())
            .unwrap(),
            @"
        export interface Name {
        }
        "
        );
    }

    #[test]
    fn test_render_properties() {
        assert_snapshot!(
            TsInterface {
                doc: None,
                name: "Name".into(),
                extensions: vec![],
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
        export interface Name {
          name: string;
          age?: number;
        }
        "
        );
    }

    #[test]
    fn test_render_indent() {
        assert_snapshot!(
            TsInterface {
                doc: None,
                name: "Name".into(),
                extensions: vec![],
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
        export interface Name {
          name: string;
          age?: number;
        }
        "
        );
    }

    #[test]
    fn test_render_extensions() {
        assert_snapshot!(
            TsInterface {
                doc: None,
                name: "Name".into(),
                extensions: vec!["Hello".into(), "World".into()],
                properties: vec![TsProperty {
                    doc: None,
                    name: "name".into(),
                    descriptor: TsDescriptor::Primitive(TsPrimitive::String),
                    required: true
                },]
            }
            .render(Default::default(), &mut Default::default())
            .unwrap(),
            @"
        export interface Name extends Hello, World {
          name: string;
        }
        "
        );
    }

    #[test]
    fn test_render_doc() {
        assert_snapshot!(
            TsInterface {
                doc: Some(TsDoc("Hello, world!".into())),
                name: "Name".into(),
                extensions: vec![],
                properties: vec![]
            }
            .render(Default::default(), &mut Default::default())
            .unwrap(),
            @"
        /** Hello, world! */
        export interface Name {
        }
        "
        );
    }
}
