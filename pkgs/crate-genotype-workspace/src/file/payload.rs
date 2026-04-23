use crate::prelude::internal::*;

pub enum GtwFilePayload {
    Config(Box<GtpConfig>),
    Module(Box<GtModuleParse>),
}
