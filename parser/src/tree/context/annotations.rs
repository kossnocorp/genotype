use crate::prelude::internal::*;

pub type GTContextAnnotation = (Option<GTDoc>, Vec<GTAttribute>);

impl GTContext {
    pub fn provide_annotation(&mut self, annotation: GTContextAnnotation) {
        self.annotation = Some(annotation);
    }

    pub fn take_annotation_or_default(&mut self) -> GTContextAnnotation {
        if let Some(annotation) = self.annotation.take() {
            annotation
        } else {
            (None, vec![])
        }
    }
}
