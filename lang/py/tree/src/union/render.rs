use genotype_lang_core_tree::indent::GTIndent;
use genotype_lang_py_config::PYLangConfig;
use genotype_lang_py_config::PYVersion;

use crate::PYRender;

use super::PYUnion;

impl PYRender for PYUnion {
    fn render(&self, indent: &GTIndent, config: &PYLangConfig) -> String {
        let content = self
            .descriptors
            .iter()
            .map(|d| d.render(indent, config))
            .collect::<Vec<String>>()
            .join(if let PYVersion::Legacy = config.version {
                ", "
            } else {
                " | "
            });

        if let PYVersion::Legacy = config.version {
            format!("Union[{}]", content)
        } else {
            content
        }
    }
}

#[cfg(test)]
mod tests {
    use genotype_lang_py_config::PYLangConfig;
    use pretty_assertions::assert_eq;

    use super::*;
    use crate::*;

    #[test]
    fn test_render_union() {
        assert_eq!(
            PYUnion {
                descriptors: vec![
                    PYDescriptor::Primitive(PYPrimitive::String),
                    PYDescriptor::Primitive(PYPrimitive::Int),
                ]
            }
            .render(&py_indent(), &Default::default()),
            "str | int"
        );
    }

    #[test]
    fn test_render_legacy() {
        assert_eq!(
            PYUnion {
                descriptors: vec![
                    PYDescriptor::Primitive(PYPrimitive::String),
                    PYDescriptor::Primitive(PYPrimitive::Int),
                ]
            }
            .render(&py_indent(), &PYLangConfig::new(PYVersion::Legacy)),
            "Union[str, int]"
        );
    }
}
