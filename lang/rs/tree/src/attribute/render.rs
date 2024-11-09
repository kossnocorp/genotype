use genotype_lang_core_tree::{indent::GTIndent, render::GTRender};

use super::RSAttribute;

impl GTRender for RSAttribute {
    fn render(&self, indent: &GTIndent) -> String {
        format!("{}#[{}]", indent.string, self.0)
    }
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use super::*;
    use crate::*;

    #[test]
    fn test_render() {
        assert_eq!(
            RSAttribute("derive".into()).render(&rs_indent()),
            "#[derive]"
        );
    }

    #[test]
    fn test_render_indent() {
        assert_eq!(
            RSAttribute("derive".into()).render(&rs_indent().increment()),
            "    #[derive]"
        );
    }
}
