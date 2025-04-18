use crate::*;
use genotype_lang_core_tree::*;
use genotype_lang_py_config::PYVersion;
use miette::Result;

impl<'a> GtlRender<'a> for PYAlias {
    type RenderContext = PYRenderContext<'a>;

    fn render(&self, context: &mut Self::RenderContext) -> Result<String> {
        let name = self.name.render(context)?;
        let descriptor = self.descriptor.render(context)?;

        let alias = if let PYVersion::Legacy = context.config.version {
            format!("{name}: TypeAlias = {descriptor}")
        } else {
            format!("type {name} = {descriptor}")
        };

        Ok(if let Some(doc) = &self.doc {
            let doc = doc.render(context)?;
            format!("{alias}\n{doc}")
        } else {
            alias
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use genotype_lang_py_config::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_render() {
        assert_eq!(
            PYAlias {
                doc: None,
                name: "Name".into(),
                descriptor: PYDescriptor::Primitive(PYPrimitive::String),
                references: vec![],
            }
            .render(&mut Default::default())
            .unwrap(),
            "type Name = str"
        );
    }

    #[test]
    fn test_render_legacy() {
        assert_eq!(
            PYAlias {
                doc: None,
                name: "Name".into(),
                descriptor: PYDescriptor::Primitive(PYPrimitive::String),
                references: vec![],
            }
            .render(&mut PYRenderContext {
                config: &PYLangConfig::new(PYVersion::Legacy),
                ..Default::default()
            })
            .unwrap(),
            "Name: TypeAlias = str"
        );
    }

    #[test]
    fn test_render_doc() {
        assert_eq!(
            PYAlias {
                doc: Some(PYDoc("Hello, world!".into())),
                name: "Name".into(),
                descriptor: PYDescriptor::Primitive(PYPrimitive::String),
                references: vec![],
            }
            .render(&mut Default::default())
            .unwrap(),
            r#"type Name = str
"""Hello, world!""""#
        );
    }
}
