use genotype_lang_py_tree::{PYContext, PYDefinition, PYOptions};

use crate::resolve::PYConvertResolve;

pub struct PYConvertContext {
    pub resolve: PYConvertResolve,
    hoist: Box<dyn Fn(PYDefinition)>,
    pub tree: PYContext,
    pub options: PYOptions,
}

impl PYConvertContext {
    pub fn new(resolve: PYConvertResolve, hoist: Box<dyn Fn(PYDefinition)>) -> Self {
        Self {
            resolve,
            hoist,
            tree: PYContext::new(),
            options: PYOptions::default(),
        }
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
            tree: PYContext::new(),
            options: PYOptions::default(),
        }
    }
}
