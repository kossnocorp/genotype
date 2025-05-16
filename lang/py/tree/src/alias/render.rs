use crate::prelude::internal::*;

impl<'a> GtlRender<'a> for PYAlias {
    type RenderState = PYRenderState;

    type RenderContext = PYRenderContext<'a>;

    fn render(
        &self,
        state: Self::RenderState,
        context: &mut Self::RenderContext,
    ) -> Result<String> {
        let name = self.name.render(state, context)?;
        let descriptor = self.descriptor.render(state, context)?;

        let alias = if let PYVersion::Legacy = context.config.version {
            format!("{name} = {descriptor}")
        } else {
            format!("type {name} = {descriptor}")
        };

        Ok(if let Some(doc) = &self.doc {
            let doc = doc.render(state, context)?;
            format!("{alias}\n{doc}")
        } else {
            alias
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
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
            .render(Default::default(), &mut Default::default())
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
            .render(
                Default::default(),
                &mut PYRenderContext {
                    config: &PyConfigLang::new(PYVersion::Legacy),
                    ..Default::default()
                }
            )
            .unwrap(),
            "Name = str"
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
            .render(Default::default(), &mut Default::default())
            .unwrap(),
            r#"type Name = str
"""Hello, world!""""#
        );
    }
}
