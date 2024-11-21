use genotype_parser::GTBranded;

use crate::visitor::GTVisitor;

use super::GTTraverse;

impl GTTraverse for GTBranded {
    fn traverse(&mut self, visitor: &mut dyn GTVisitor) {
        visitor.visit_branded(self);

        let identifier = match self {
            GTBranded::Boolean(_, _, identifier) => identifier,
            GTBranded::String(_, _, identifier) => identifier,
            GTBranded::Int(_, _, identifier) => identifier,
            GTBranded::Float(_, _, identifier) => identifier,
            GTBranded::Null(_, _, identifier) => identifier,
        };

        identifier.traverse(visitor);
    }
}
