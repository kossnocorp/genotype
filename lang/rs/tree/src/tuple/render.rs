use genotype_lang_core_tree::indent::GTIndent;
use genotype_lang_rs_config::RSLangConfig;

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
        format!("({descriptors})")
    }
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

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
            "(String, isize)"
        );
    }

    #[test]
    fn test_render_empty_tuple() {
        assert_eq!(
            RSTuple {
                descriptors: vec![]
            }
            .render(&rs_indent(), &Default::default()),
            "()"
        );
    }
}
