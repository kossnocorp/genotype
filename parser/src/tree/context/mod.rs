use super::{GTObjectNameParent, GTResolve};

pub struct GTContext {
    pub resolve: GTResolve,
    pub object_parent: Option<GTObjectNameParent>,
}

impl GTContext {
    pub fn new() -> Self {
        GTContext {
            resolve: GTResolve::new(),
            object_parent: None,
        }
    }
}
