use genotype_lang_core_tree::{indent::GTIndent, render::GTRender};

use super::TSReference;

impl GTRender for TSReference {
    fn render(&self, indent: &GTIndent) -> String {
        match self {
            TSReference::Unresolved(name) => name,
            TSReference::Local(name) => name,
            TSReference::External(name, _) => name,
        }
        .render(indent)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{indent::ts_indent, path::TSPath};

    #[test]
    fn test_render_unresolved() {
        assert_eq!(
            TSReference::Unresolved("Foo".into()).render(&ts_indent()),
            "Foo"
        );
    }

    #[test]
    fn test_render_resolved() {
        assert_eq!(
            TSReference::External(
                "foo.Bar".into(),
                TSPath::Resolved("./path/to/module.ts".into())
            )
            .render(&ts_indent()),
            "foo.Bar"
        );
    }
}
