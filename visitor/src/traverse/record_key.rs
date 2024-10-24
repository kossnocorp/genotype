use genotype_parser::*;

use crate::visitor::GTVisitor;

use super::GTTraverse;

impl GTTraverse for GTRecordKey {
    fn traverse(&mut self, visitor: &mut dyn GTVisitor) {
        visitor.visit_record_key(self);
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
        let mut key = GTRecordKey::String((0, 0).into());
        key.traverse(&mut visitor);
        assert_eq!(
            visitor.visited,
            vec![GTMockVisited::RecordKey(key.clone()),]
        );
    }
}
