use crate::prelude::internal::*;

pub trait GtlConfigHealth {
    fn health_check(&self) -> Vec<GtlConfigNotice> {
        vec![]
    }
}
