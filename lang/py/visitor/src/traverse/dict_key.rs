use genotype_lang_py_tree::PYDictKey;

use crate::visitor::PYVisitor;

use super::PYTraverse;

impl PYTraverse for PYDictKey {
    fn traverse(&mut self, visitor: &mut dyn PYVisitor) {
        visitor.visit_dict_key(self);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::visitor::mock::*;
    use genotype_lang_py_tree::PYDictKey;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_traverse() {
        let mut visitor = PYMockVisitor::new();
        let mut key = PYDictKey::String;
        key.traverse(&mut visitor);
        assert_eq!(visitor.visited, vec![PYMockVisited::DictKey(key.clone()),]);
    }
}