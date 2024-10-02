use genotype_lang_core_tree::{indent::GTIndent, render::GTRender};

use super::TSPath;

impl GTRender for TSPath {
    fn render(&self, _indent: &GTIndent) -> String {
        match self {
            TSPath::Unresolved(path) => path,
            TSPath::Resolved(path) => path,
        }
        .clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::indent::ts_indent;

    #[test]
    fn test_render_unresolved() {
        assert_eq!(
            TSPath::Unresolved("./path/to/module".into()).render(&ts_indent()),
            "./path/to/module"
        );
    }

    #[test]
    fn test_render_resolved() {
        assert_eq!(
            TSPath::Resolved("./path/to/module.ts".into()).render(&ts_indent()),
            "./path/to/module.ts"
        );
    }
}
