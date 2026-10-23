use crate::prelude::internal::*;

mod annotations;
pub use annotations::*;

mod ids;

mod naming;

mod generics;

#[derive(Default, Debug, PartialEq, Clone)]
pub struct GtContext {
    /// Current module id.
    pub module_id: GtModuleId,
    pub resolve: GtModuleResolve,
    pub named_parents: Vec<GtContextParent>,
    /// A set of taken definition names. It allows to generate unique synthetic
    /// names.
    // [TODO] Use `GtNamingContext` instead of `claimed_names` in the future.
    pub claimed_names: IndexSet<String>,
    pub annotation: Option<GtContextAnnotation>,
    /// A stack of generic parameters for the current context. It allows to resolve
    /// generic parameters in nested contexts.
    pub generics_scope: Vec<Vec<GtGenericParameter>>,
}

/// The parent context enum that defines the kind of a parent an object has.
/// It allows building object names from the parents.
#[derive(Debug, PartialEq, Clone)]
pub enum GtContextParent {
    /// An explicitly named alias parent.
    Alias(GtIdentifier),
    /// An anonymous parent, i.e. an union or a nested object.
    Anonymous,
    /// A property parent.
    Property(GtKey),
}

impl GtContext {
    pub fn new(module_id: GtModuleId) -> Self {
        GtContext {
            module_id,
            resolve: GtModuleResolve::new(),
            named_parents: vec![],
            claimed_names: IndexSet::new(),
            annotation: None,
            generics_scope: vec![],
        }
    }
}
