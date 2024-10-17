use genotype_lang_py_tree::PYDefinition;

use crate::resolve::PYConvertResolve;

pub struct PYConvertContext {
    pub resolve: PYConvertResolve,
    hoist: Box<dyn Fn(PYDefinition)>,
}

impl PYConvertContext {
    pub fn new(resolve: PYConvertResolve, hoist: Box<dyn Fn(PYDefinition)>) -> Self {
        Self { resolve, hoist }
    }

    pub fn hoist(&self, definition: PYDefinition) {
        (self.hoist)(definition);
    }
}

impl Default for PYConvertContext {
    fn default() -> Self {
        Self {
            resolve: PYConvertResolve::new(),
            hoist: Box::new(|_| {}),
        }
    }
}
