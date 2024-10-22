use genotype_parser::*;

use crate::visitor::GTVisitor;

use super::GTTraverse;

impl GTTraverse for GTAttributeName {
    fn traverse(&mut self, visitor: &mut dyn GTVisitor) {
        visitor.visit_attribute_name(self);
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
        let mut name = GTAttributeName::new((0, 0).into(), "answer".into());
        name.traverse(&mut visitor);
        assert_eq!(
            visitor.visited,
            vec![GTMockVisited::AttributeName(name.clone()),]
        );
    }
}
