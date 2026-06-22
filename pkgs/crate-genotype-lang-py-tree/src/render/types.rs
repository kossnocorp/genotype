use crate::prelude::internal::*;

pub struct PyRenderTypes;

impl<'context> GtlRenderTypes<'context> for PyRenderTypes {
    type State = PyRenderState;
    type Context = PyRenderContext<'context>;
    type Error = PyRenderError;
}
