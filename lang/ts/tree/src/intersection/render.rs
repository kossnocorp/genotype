use genotype_lang_core_tree::{indent::GTIndent, render::GTRender};

use super::TSIntersection;

impl GTRender for TSIntersection {
    fn render(&self, indent: &GTIndent) -> String {
        self.descriptors
            .iter()
            .map(|d| d.render(indent))
            .collect::<Vec<String>>()
            .join(" & ")
    }
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use super::*;
    use crate::*;

    #[test]
    fn test_render_union() {
        assert_eq!(
            TSIntersection {
                descriptors: vec![
                    TSObject {
                        properties: vec![TSProperty {
                            doc: None,
                            name: "hello".into(),
                            descriptor: TSPrimitive::String.into(),
                            required: true,
                        }],
                    }
                    .into(),
                    "World".into(),
                ]
            }
            .render(&ts_indent()),
            r#"{
  hello: string
} & World"#
        );
    }
}
