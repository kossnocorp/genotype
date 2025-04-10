use crate::*;
use genotype_lang_core_tree::*;
use miette::Result;

impl<'a> GtlRender<'a> for TSInterface {
    type RenderContext = TSRenderContext<'a>;

    fn render(&self, context: &mut Self::RenderContext) -> Result<String> {
        let mut prop_indent = context.indent_inc();

        let properties = self
            .properties
            .iter()
            .map(|property| property.render(&mut prop_indent))
            .collect::<Result<Vec<_>>>()?
            .iter()
            .map(|property| format!("{property};"))
            .collect::<Vec<_>>()
            .join("\n");

        let extensions = self
            .extensions
            .iter()
            .map(|extension| extension.render(context))
            .collect::<Result<Vec<_>>>()?
            .join(", ");

        let name = self.name.render(context)?;
        let extends = if extensions.len() > 0 {
            format!(" extends {}", extensions)
        } else {
            "".into()
        };

        TSDoc::with_doc(
            &self.doc,
            context,
            format!(
                "{}export interface {name}{extends} {{\n{properties}{}{}",
                context.indent_legacy.string,
                if properties.len() > 0 { "\n" } else { "" },
                context.indent_legacy.format("}")
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
            .render(&mut Default::default())
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
            .render(&mut Default::default())
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
            .render(&mut TSRenderContext::default().indent_inc())
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
            .render(&mut Default::default())
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
            .render(&mut Default::default())
            .unwrap(),
            r#"/** Hello, world! */
export interface Name {
}"#
        );
    }
}
