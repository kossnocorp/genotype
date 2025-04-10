use crate::*;
use genotype_lang_core_tree::*;
use miette::Result;

impl<'a> GtlRender<'a> for TSObject {
    type RenderContext = TSRenderContext<'a>;

    fn render(&self, context: &mut Self::RenderContext) -> Result<String> {
        let mut prop_indent = context.indent_inc();
        let properties = self
            .properties
            .iter()
            .map(|property| property.render(&mut prop_indent))
            .collect::<Result<Vec<_>>>()?
            .join(",\n");

        Ok(format!(
            "{{\n{properties}{}{}",
            if properties.len() > 0 { "\n" } else { "" },
            context.indent_legacy.format("}")
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_render_empty() {
        assert_eq!(
            TSObject { properties: vec![] }
                .render(&mut Default::default())
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
            .render(&mut Default::default())
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
            .render(&mut TSRenderContext::default().indent_inc())
            .unwrap(),
            "{\n    name: string,\n    age?: number\n  }"
        );
    }
}
