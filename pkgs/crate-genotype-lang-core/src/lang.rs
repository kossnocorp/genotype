use crate::prelude::internal::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize)]
pub enum GtLang {
    Py,
    Rs,
    Ts,
}
