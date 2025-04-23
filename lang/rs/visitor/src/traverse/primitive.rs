use genotype_lang_rs_tree::*;

use crate::visitor::RSVisitor;

use super::RSTraverse;

impl RSTraverse for RSPrimitive {
    fn traverse(&mut self, visitor: &mut dyn RSVisitor) {
        visitor.visit_primitive(self);
    }
}
