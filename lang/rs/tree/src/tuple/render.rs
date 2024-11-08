use genotype_lang_core_tree::indent::GTIndent;
use genotype_lang_rs_config::RSLangConfig;
use genotype_lang_rs_config::RSVersion;

use crate::RSRender;

use super::RSTuple;

impl RSRender for RSTuple {
    fn render(&self, indent: &GTIndent, config: &RSLangConfig) -> String {
        let descriptors = self
            .descriptors
            .iter()
            .map(|d| d.render(indent, config))
            .collect::<Vec<String>>()
            .join(", ");
        format!(
            "{}{}{}{}",
            if let RSVersion::Legacy = config.version {
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
    use genotype_lang_rs_config::RSLangConfig;
    use pretty_assertions::assert_eq;

    use super::*;
    use crate::*;

    #[test]
    fn test_render_tuple() {
        assert_eq!(
            RSTuple {
                descriptors: vec![
                    RSDescriptor::Primitive(RSPrimitive::String),
                    RSDescriptor::Primitive(RSPrimitive::Int),
                ]
            }
            .render(&rs_indent(), &Default::default()),
            "tuple[str, int]"
        );
    }

    #[test]
    fn test_render_empty_tuple() {
        assert_eq!(
            RSTuple {
                descriptors: vec![]
            }
            .render(&rs_indent(), &Default::default()),
            "tuple[()]"
        );
    }

    #[test]
    fn test_render_legacy() {
        assert_eq!(
            RSTuple {
                descriptors: vec![]
            }
            .render(&rs_indent(), &RSLangConfig::new(RSVersion::Legacy)),
            "Tuple[()]"
        );
    }
}
