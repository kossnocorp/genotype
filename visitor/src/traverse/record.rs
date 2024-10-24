use genotype_parser::*;

use crate::visitor::GTVisitor;

use super::GTTraverse;

impl GTTraverse for GTRecord {
    fn traverse(&mut self, visitor: &mut dyn GTVisitor) {
        visitor.visit_record(self);

        self.key.traverse(visitor);
        self.descriptor.traverse(visitor);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::visitor::mock::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_traverse() {
        let mut visitor = GTMockVisitor::new();
        let key = GTRecordKey::String((0, 0).into());
        let primitive = GTPrimitive::String((0, 0).into());
        let descriptor = GTDescriptor::Primitive(primitive.clone());
        let mut record = GTRecord {
            span: (0, 0).into(),
            key: key.clone(),
            descriptor: descriptor.clone(),
        };
        record.traverse(&mut visitor);
        assert_eq!(
            visitor.visited,
            vec![
                GTMockVisited::Record(record.clone()),
                GTMockVisited::RecordKey(key.clone()),
                GTMockVisited::Descriptor(descriptor.clone()),
                GTMockVisited::Primitive(primitive.clone()),
            ]
        );
    }
}
