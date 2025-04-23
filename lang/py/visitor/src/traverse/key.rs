use genotype_lang_py_tree::*;

use crate::visitor::PYVisitor;

use super::PYTraverse;

impl PYTraverse for PYKey {
    fn traverse(&mut self, visitor: &mut dyn PYVisitor) {
        visitor.visit_key(self);
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
        let mut key = PYKey("name".into());
        key.traverse(&mut visitor);
        assert_eq!(visitor.visited, vec![PYMockVisited::Key(key.clone()),]);
    }
}
