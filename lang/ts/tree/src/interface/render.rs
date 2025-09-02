use crate::prelude::internal::*;

impl<'a> GtlRender<'a> for TSInterface {
    type RenderState = TSRenderState;

    type RenderContext = TSRenderContext<'a>;

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

        TSDoc::with_doc(
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
    use pretty_assertions::assert_eq;

    #[test]
    fn test_render_empty() {
        assert_eq!(
            TSInterface {
                doc: None,
                name: "Name".into(),
                extensions: vec![],
                properties: vec![]
            }
            .render(Default::default(), &mut Default::default())
            .unwrap(),
            "export interface Name {\n}"
        );
    }

    #[test]
    fn test_render_properties() {
        assert_eq!(
            TSInterface {
                doc: None,
                name: "Name".into(),
                extensions: vec![],
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
            r#"export interface Name {
  name: string;
  age?: number;
}"#
        );
    }

    #[test]
    fn test_render_indent() {
        assert_eq!(
            TSInterface {
                doc: None,
                name: "Name".into(),
                extensions: vec![],
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
            r#"  export interface Name {
    name: string;
    age?: number;
  }"#
        );
    }

    #[test]
    fn test_render_extensions() {
        assert_eq!(
            TSInterface {
                doc: None,
                name: "Name".into(),
                extensions: vec!["Hello".into(), "World".into()],
                properties: vec![TSProperty {
                    doc: None,
                    name: "name".into(),
                    descriptor: TSDescriptor::Primitive(TSPrimitive::String),
                    required: true
                },]
            }
            .render(Default::default(), &mut Default::default())
            .unwrap(),
            r#"export interface Name extends Hello, World {
  name: string;
}"#
        );
    }

    #[test]
    fn test_render_doc() {
        assert_eq!(
            TSInterface {
                doc: Some(TSDoc("Hello, world!".into())),
                name: "Name".into(),
                extensions: vec![],
                properties: vec![]
            }
            .render(Default::default(), &mut Default::default())
            .unwrap(),
            r#"/** Hello, world! */
export interface Name {
}"#
        );
    }
}
