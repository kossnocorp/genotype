use genotype_parser::tree::path::GTPath;

use crate::visitor::GTVisitor;

use super::GTTraverse;

impl GTTraverse for GTPath {
    fn traverse(&mut self, visitor: &mut dyn GTVisitor) {
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
        let mut visitor = GTMockVisitor::new();
        let mut path = GTPath::new((0, 0).into(), "./path/to/module");
        path.traverse(&mut visitor);
        assert_eq!(visitor.visited, vec![GTMockVisited::Path(path.clone()),]);
    }
}
