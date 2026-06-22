use crate::prelude::internal::*;

mod types;
pub use types::*;

mod context;
pub use context::*;

mod state;
pub use state::*;

mod module;
pub use module::*;

pub trait GtlRender<'context, Types: GtlRenderTypes<'context>> {
    fn render(
        &self,
        state: Types::State,
        context: &mut Types::Context,
    ) -> Result<String, Types::Error>;
}
