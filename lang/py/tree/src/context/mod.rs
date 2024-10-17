use std::collections::HashSet;

use crate::{PYIdentifier, PYOptions, PYPath};

#[derive(Debug, PartialEq, Clone)]
pub struct PYContext {
    pub imports: HashSet<(PYPath, PYIdentifier)>,
}

impl PYContext {
    pub fn new() -> Self {
        PYContext {
            imports: HashSet::new(),
        }
    }
}

pub trait PYContextResolve {
    fn resolve(self, _context: &mut PYContext, _options: &PYOptions) -> Self
    where
        Self: Sized,
    {
        self
    }
}
