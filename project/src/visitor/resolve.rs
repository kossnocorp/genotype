use genotype_visitor::visitor::GTVisitor;

pub struct GTProjectResolveVisitor {}

impl GTProjectResolveVisitor {
    pub fn new() -> Self {
        Self {}
    }
}

impl GTVisitor for GTProjectResolveVisitor {}
