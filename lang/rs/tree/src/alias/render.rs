use genotype_lang_core_tree::indent::GTIndent;
use genotype_lang_rs_config::RSLangConfig;
use miette::Result;

use crate::RSRender;

use super::RSAlias;

impl RSRender for RSAlias {
    fn render(&self, indent: &GTIndent, config: &RSLangConfig) -> Result<String> {
        let name = self.name.render(indent, config)?;
        let descriptor = self.descriptor.render(indent, config)?;
        let r#type = format!("type {name} = {descriptor};");

        Ok(if let Some(doc) = &self.doc {
            format!("{}\n{}", doc.render(indent, config)?, r#type)
        } else {
            r#type
        })
    }
}

#[cfg(test)]
mod tests {
    use genotype_parser::GTDefinitionId;
    use pretty_assertions::assert_eq;

    use crate::*;

    #[test]
    fn test_render() {
        assert_eq!(
            RSAlias {
                id: GTDefinitionId("module".into(), "Name".into()),
                doc: None,
                name: "Name".into(),
                descriptor: RSDescriptor::Primitive(RSPrimitive::String),
            }
            .render(&rs_indent(), &Default::default())
            .unwrap(),
            "type Name = String;"
        );
    }

    #[test]
    fn test_render_doc() {
        assert_eq!(
            RSAlias {
                id: GTDefinitionId("module".into(), "Name".into()),
                doc: Some("Hello, world!".into()),
                name: "Name".into(),
                descriptor: RSDescriptor::Primitive(RSPrimitive::String),
            }
            .render(&rs_indent(), &Default::default())
            .unwrap(),
            r#"/// Hello, world!
type Name = String;"#
        );
    }
}
