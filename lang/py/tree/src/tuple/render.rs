use genotype_lang_core_tree::indent::GTIndent;
use genotype_lang_py_config::PYLangConfig;
use genotype_lang_py_config::PYVersion;

use crate::PYRender;

use super::PYTuple;

impl PYRender for PYTuple {
    fn render(&self, indent: &GTIndent, config: &PYLangConfig) -> String {
        let descriptors = self
            .descriptors
            .iter()
            .map(|d| d.render(indent, config))
            .collect::<Vec<String>>()
            .join(", ");
        format!(
            "{}{}{}{}",
            if let PYVersion::Legacy = config.version {
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
    use genotype_lang_py_config::PYLangConfig;
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
            .render(&py_indent(), &PYLangConfig::new(PYVersion::Legacy)),
            "Tuple[()]"
        );
    }
}
