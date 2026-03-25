use crate::prelude::internal::*;

impl<'a> GtlRender<'a> for PyDefinition {
    type RenderState = PyRenderState;

    type RenderContext = PyRenderContext<'a>;

    fn render(
        &self,
        state: Self::RenderState,
        context: &mut Self::RenderContext,
    ) -> Result<String> {
        match self {
            PyDefinition::Alias(alias) => alias.render(state, context),
            PyDefinition::Class(interface) => interface.render(state, context),
            PyDefinition::Newtype(newtype) => newtype.render(state, context),
            PyDefinition::Embed(embed) => embed.render(state, context),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use insta::assert_snapshot;

    #[test]
    fn test_render_alias() {
        assert_snapshot!(
            PyDefinition::Alias(PyAlias {
                doc: None,
                name: "Name".into(),
                descriptor: PyDescriptor::Primitive(PyPrimitive::String),
                references: vec![],
            })
            .render(Default::default(), &mut Default::default())
            .unwrap(),
            @"type Name = str"
        );
    }

    #[test]
    fn test_render_class() {
        assert_snapshot!(
            PyDefinition::Class(PyClass {
                doc: None,
                name: "Name".into(),
                extensions: vec![],
                properties: vec![
                    PyProperty {
                        doc: None,
                        name: "name".into(),
                        descriptor: PyDescriptor::Primitive(PyPrimitive::String),
                        required: true
                    },
                    PyProperty {
                        doc: None,
                        name: "age".into(),
                        descriptor: PyDescriptor::Primitive(PyPrimitive::Int),
                        required: false
                    }
                ],
                references: vec![],
            })
            .render(Default::default(), &mut Default::default())
            .unwrap(),
            @"
        class Name(Model):
            name: str
            age: Optional[int] = None
        "
        );
    }

    #[test]
    fn test_render_branded() {
        assert_snapshot!(
            PyDefinition::Newtype(PyNewtype {
                doc: None,
                name: "UserId".into(),
                primitive: PyPrimitive::String,
            })
            .render(Default::default(), &mut Default::default())
            .unwrap(),
            @r#"UserId = NewType("UserId", str)"#
        );
    }

    #[test]
    fn test_render_embed() {
        assert_snapshot!(
            PyDefinition::Embed(PyEmbedDefinition {
                name: "Name".into(),
                embed: r#"class Hello:\n    name = "World""#
                    .into()
            })
            .render(Default::default(), &mut Default::default())
            .unwrap(),
            @r#"class Hello:\n    name = "World""#
        );
    }
}
