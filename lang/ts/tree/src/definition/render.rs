use crate::*;
use genotype_lang_core_tree::*;
use miette::Result;

impl<'a> GtlRender<'a> for TSDefinition {
    type RenderContext = TSRenderContext<'a>;

    fn render(&self, context: &mut Self::RenderContext) -> Result<String> {
        match self {
            TSDefinition::Alias(alias) => alias.render(context),
            TSDefinition::Interface(interface) => interface.render(context),
            TSDefinition::Branded(branded) => branded.render(context),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_render_alias() {
        assert_eq!(
            TSDefinition::Alias(TSAlias {
                doc: None,
                name: "Name".into(),
                descriptor: TSDescriptor::Primitive(TSPrimitive::String),
            })
            .render(&mut Default::default())
            .unwrap(),
            "export type Name = string;"
        );
    }

    #[test]
    fn test_render_interface() {
        assert_eq!(
            TSDefinition::Interface(TSInterface {
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
            })
            .render(&mut Default::default())
            .unwrap(),
            r#"export interface Name {
  name: string;
  age?: number;
}"#
        );
    }

    #[test]
    fn test_render_branded() {
        assert_eq!(
            TSDefinition::Branded(TSBranded {
                doc: None,
                name: "Version".into(),
                primitive: TSPrimitive::Number
            })
            .render(&mut Default::default())
            .unwrap(),
            r#"export type Version = number & { [versionBrand]: true };
declare const versionBrand: unique symbol;"#
        );
    }
}
