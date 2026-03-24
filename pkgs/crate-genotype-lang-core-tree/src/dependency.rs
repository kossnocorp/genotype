use std::hash::Hash;

pub trait GtlDependency {}

pub trait GtlDependencyIdent: Hash + Eq {}

pub trait GtlDependencyRef {}
