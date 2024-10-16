use genotype_lang_core_tree::indent::GTIndent;

use crate::{PYOptions, PYRender};

use super::PYDefinition;

impl PYRender for PYDefinition {
    fn render(&self, indent: &GTIndent, options: &PYOptions) -> String {
        match self {
            PYDefinition::Alias(alias) => alias.render(indent, options),
            PYDefinition::Class(interface) => interface.render(indent, options),
        }
    }
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use crate::*;

    #[test]
    fn test_render_alias() {
        assert_eq!(
            PYDefinition::Alias(PYAlias {
                name: "Name".into(),
                descriptor: PYDescriptor::Primitive(PYPrimitive::String),
            })
            .render(&py_indent(), &PYOptions::default()),
            "type Name = str"
        );
    }

    #[test]
    fn test_render_class() {
        assert_eq!(
            PYDefinition::Class(PYClass {
                name: "Name".into(),
                extensions: vec![],
                properties: vec![
                    PYProperty {
                        name: "name".into(),
                        descriptor: PYDescriptor::Primitive(PYPrimitive::String),
                        required: true
                    },
                    PYProperty {
                        name: "age".into(),
                        descriptor: PYDescriptor::Primitive(PYPrimitive::Int),
                        required: false
                    }
                ]
            })
            .render(&py_indent(), &PYOptions::default()),
            r#"@dataclass
class Name:
    name: str
    age: Optional[int] = None"#
        );
    }
}
