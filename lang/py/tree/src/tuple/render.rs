use genotype_config::GTConfig;
use genotype_lang_core_tree::indent::GTIndent;
use genotype_lang_py_config::PYVersion;

use crate::PYRender;

use super::PYTuple;

impl PYRender for PYTuple {
    fn render(&self, indent: &GTIndent, config: &GTConfig) -> String {
        let descriptors = self
            .descriptors
            .iter()
            .map(|d| d.render(indent, config))
            .collect::<Vec<String>>()
            .join(", ");
        format!(
            "{}{}{}{}",
            if let PYVersion::Legacy = config.python_version() {
                "Tuple"
            } else {
                "tuple"
            },
            "[",
            if descriptors.len() > 0 {
                descriptors
            } else {
                "()".into()
            },
            "]"
        )
    }
}

#[cfg(test)]
mod tests {
    use genotype_lang_py_config::PYConfig;
    use pretty_assertions::assert_eq;

    use super::*;
    use crate::*;

    #[test]
    fn test_render_tuple() {
        assert_eq!(
            PYTuple {
                descriptors: vec![
                    PYDescriptor::Primitive(PYPrimitive::String),
                    PYDescriptor::Primitive(PYPrimitive::Int),
                ]
            }
            .render(&py_indent(), &Default::default()),
            "tuple[str, int]"
        );
    }

    #[test]
    fn test_render_empty_tuple() {
        assert_eq!(
            PYTuple {
                descriptors: vec![]
            }
            .render(&py_indent(), &Default::default()),
            "tuple[()]"
        );
    }

    #[test]
    fn test_render_legacy() {
        assert_eq!(
            PYTuple {
                descriptors: vec![]
            }
            .render(
                &py_indent(),
                &GTConfig::default().with_python(PYConfig::new(PYVersion::Legacy))
            ),
            "Tuple[()]"
        );
    }
}
