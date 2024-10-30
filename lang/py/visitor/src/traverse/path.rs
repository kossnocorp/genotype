use genotype_lang_py_tree::path::PYPath;

use crate::visitor::PYVisitor;

use super::PYTraverse;

impl PYTraverse for PYPath {
    fn traverse(&mut self, visitor: &mut dyn PYVisitor) {
        visitor.visit_path(self);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::visitor::mock::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_traverse() {
        let mut visitor = PYMockVisitor::new();
        let mut path = PYPath("./path/to/module".into());
        path.traverse(&mut visitor);
        assert_eq!(visitor.visited, vec![PYMockVisited::Path(path.clone()),]);
    }
}
