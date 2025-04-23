use genotype_lang_py_tree::*;

use crate::visitor::PYVisitor;

use super::PYTraverse;

impl PYTraverse for PYAny {
    fn traverse(&mut self, visitor: &mut dyn PYVisitor) {
        visitor.visit_any(self);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::visitor::mock::*;
    use genotype_lang_py_tree::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_traverse() {
        let mut visitor = PYMockVisitor::new();
        let mut any = PYAny;
        any.traverse(&mut visitor);
        assert_eq!(visitor.visited, vec![PYMockVisited::Any(any.clone()),]);
    }
}
