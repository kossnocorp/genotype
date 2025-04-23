use genotype_lang_py_tree::*;

use crate::visitor::PYVisitor;

use super::PYTraverse;

impl PYTraverse for PYDict {
    fn traverse(&mut self, visitor: &mut dyn PYVisitor) {
        visitor.visit_dict(self);

        self.key.traverse(visitor);
        self.descriptor.traverse(visitor);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::visitor::mock::{PYMockVisited, PYMockVisitor};
    use genotype_lang_py_tree::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_traverse() {
        let mut visitor = PYMockVisitor::new();
        let key = PYDictKey::String;
        let primitive = PYPrimitive::String;
        let descriptor = PYDescriptor::Primitive(primitive.clone());
        let mut dict = PYDict {
            key: key.clone(),
            descriptor: descriptor.clone(),
        };
        dict.traverse(&mut visitor);
        assert_eq!(
            visitor.visited,
            vec![
                PYMockVisited::Dict(dict.clone()),
                PYMockVisited::DictKey(key.clone()),
                PYMockVisited::Descriptor(descriptor.clone()),
                PYMockVisited::Primitive(primitive.clone()),
            ]
        );
    }
}
