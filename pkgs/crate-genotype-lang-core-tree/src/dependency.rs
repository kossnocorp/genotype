use crate::prelude::internal::*;

pub trait GtlDependency {
    type Ident: GtlDependencyIdent;
}

pub trait GtlDependencyIdent: Clone + PartialEq + Eq + Hash {
    type Path: GtlPath;

    fn as_path(&self) -> Self::Path;
}

// impl PartialEq for dyn GtlDependencyIdent {
//     fn eq(&self, other: &Self) -> bool {
//         self.as_path_str() == other.as_path_str()
//     }
// }

// impl Eq for dyn GtlDependencyIdent {}

// impl Hash for dyn GtlDependencyIdent {
//     fn hash<H: Hasher>(&self, state: &mut H) {
//         self.as_path_str().hash(state);
//     }
// }
