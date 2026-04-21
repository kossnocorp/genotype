use crate::prelude::internal::*;

pub enum GtwFilePayload {
    Config(Box<GtConfig>),
    Module(Box<GtModuleParse>),
}
