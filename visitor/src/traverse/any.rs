use genotype_parser::{tree::key::GTKey, GTAny};

use crate::visitor::GTVisitor;

use super::GTTraverse;

impl GTTraverse for GTAny {
    fn traverse(&mut self, visitor: &mut dyn GTVisitor) {
        visitor.visit_any(self);
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
        let mut key = GTKey((0, 0).into(), "name".into());
        key.traverse(&mut visitor);
        assert_eq!(visitor.visited, vec![GTMockVisited::Key(key.clone()),]);
    }
}
