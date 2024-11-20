use crate::{RSDependency, RSIdentifier};

#[cfg(test)]
pub mod mock;

#[derive(PartialEq)]
pub enum RSContextRenderDeriveMode {
    Struct,
    UnionEnum,
}

pub trait RSContext {
    fn import(&mut self, dependency: RSDependency, name: RSIdentifier);

    fn render_derive(&self, _mode: RSContextRenderDeriveMode) -> String {
        String::new()
    }
}
