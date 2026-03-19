use crate::prelude::internal::*;

#[derive(Debug, PartialEq, Clone)]
pub struct GTContextAnnotation {
    pub doc: Option<GTDoc>,
    pub attributes: Vec<GTAttribute>,
}

impl GTContext {
    pub fn provide_annotation(&mut self, annotation: GTContextAnnotation) {
        self.annotation = Some(annotation);
    }

    pub fn take_annotation(&mut self) -> Option<GTContextAnnotation> {
        self.annotation.take()
    }
}
