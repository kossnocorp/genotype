use genotype_lang_core::{indent::GTIndent, render::GTRender};

use super::TSName;

impl GTRender for TSName {
    fn render(&self, _indent: &GTIndent) -> String {
        self.0.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::indent::ts_indent;

    #[test]
    fn test_render_name() {
        let indent = ts_indent();
        assert_eq!(TSName("foo".to_string()).render(&indent), "foo");
    }
}
