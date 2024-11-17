use crate::{RSDependency, RSIdentifier};

#[cfg(test)]
pub mod mock;

pub trait RSContext {
    fn import(&mut self, dependency: RSDependency, name: RSIdentifier);

    fn render_derive(&self) -> String {
        String::new()
    }
}
