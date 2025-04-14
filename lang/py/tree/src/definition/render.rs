use crate::*;
use genotype_lang_core_tree::*;
use miette::Result;

impl<'a> GtlRender<'a> for PYDefinition {
    type RenderState = PYRenderState;

    type RenderContext = PYRenderContext<'a>;

    fn render(&self, state: Self::RenderState, context: &mut Self::RenderContext) -> Result<String> {
        match self {
            PYDefinition::Alias(alias) => alias.render(state, context),
            PYDefinition::Class(interface) => interface.render(state, context),
            PYDefinition::Newtype(newtype) => newtype.render(state, context),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_render_alias() {
        assert_eq!(
            PYDefinition::Alias(PYAlias {
                doc: None,
                name: "Name".into(),
                descriptor: PYDescriptor::Primitive(PYPrimitive::String),
                references: vec![],
            })
            .render(Default::default(), &mut Default::default())
            .unwrap(),
            "type Name = str"
        );
    }

    #[test]
    fn test_render_class() {
        assert_eq!(
            PYDefinition::Class(PYClass {
                doc: None,
                name: "Name".into(),
                extensions: vec![],
                properties: vec![
                    PYProperty {
                        doc: None,
                        name: "name".into(),
                        descriptor: PYDescriptor::Primitive(PYPrimitive::String),
                        required: true
                    },
                    PYProperty {
                        doc: None,
                        name: "age".into(),
                        descriptor: PYDescriptor::Primitive(PYPrimitive::Int),
                        required: false
                    }
                ],
                references: vec![],
            })
            .render(Default::default(), &mut Default::default())
            .unwrap(),
            r#"class Name(Model):
    name: str
    age: Optional[int] = None"#
        );
    }

    #[test]
    fn test_render_branded() {
        assert_eq!(
            PYDefinition::Newtype(PYNewtype {
                doc: None,
                name: "UserId".into(),
                primitive: PYPrimitive::String,
            })
            .render(Default::default(), &mut Default::default())
            .unwrap(),
            r#"UserId = NewType("UserId", str)"#
        );
    }
}
