use crate::prelude::internal::*;

mod state;
pub use state::*;

mod source_state;
pub use source_state::*;

#[derive(Debug, Clone, PartialEq, Error, Diagnostic, Serialize)]
pub enum GtlProjectModuleRenderError {
    #[error("Can't render module from source state: {source_state_name}")]
    State { source_state_name: String },

    #[error("Failed to render module")]
    RenderError {
        #[source]
        #[diagnostic_source]
        error: Box<dyn GtlError>,
    },
}
