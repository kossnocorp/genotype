use genotype_lang_core_tree::indent::GTIndent;
use genotype_lang_rs_config::RSLangConfig;
use miette::Result;

use crate::RSRender;

use super::RSTuple;

impl RSRender for RSTuple {
    fn render(&self, indent: &GTIndent, config: &RSLangConfig) -> Result<String> {
        let descriptors = self
            .descriptors
            .iter()
            .map(|d| d.render(indent, config))
            .collect::<Result<Vec<String>>>()?
            .join(", ");
        Ok(format!("({descriptors})"))
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
                    RSDescriptor::Primitive(RSPrimitive::IntSize),
                ]
            }
            .render(&rs_indent(), &Default::default())
            .unwrap(),
            "(String, isize)"
        );
    }

    #[test]
    fn test_render_empty_tuple() {
        assert_eq!(
            RSTuple {
                descriptors: vec![]
            }
            .render(&rs_indent(), &Default::default())
            .unwrap(),
            "()"
        );
    }
}
