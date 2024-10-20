use genotype_config::GTConfig;
use genotype_lang_core_tree::indent::GTIndent;
use genotype_lang_py_config::PYVersion;

use crate::PYRender;

use super::PYUnion;

impl PYRender for PYUnion {
    fn render(&self, indent: &GTIndent, config: &GTConfig) -> String {
        let content = self
            .descriptors
            .iter()
            .map(|d| d.render(indent, config))
            .collect::<Vec<String>>()
            .join(if let PYVersion::Legacy = config.python_version() {
                ", "
            } else {
                " | "
            });

        if let PYVersion::Legacy = config.python_version() {
            format!("Union[{}]", content)
        } else {
            content
        }
    }
}

#[cfg(test)]
mod tests {
    use genotype_lang_py_config::PYConfig;
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
            .render(
                &py_indent(),
                &GTConfig::default().with_python(PYConfig::new(PYVersion::Legacy))
            ),
            "Union[str, int]"
        );
    }
}
