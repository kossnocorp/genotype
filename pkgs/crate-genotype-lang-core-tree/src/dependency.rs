use crate::prelude::internal::*;

pub trait GtlDependency {
    type Ident: GtlDependencyIdent;
}

pub trait GtlDependencyIdent: Clone + PartialEq + Eq + Hash {}
