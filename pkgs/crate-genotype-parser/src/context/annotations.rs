use crate::prelude::internal::*;

pub type GtContextAnnotation = (Option<GtDoc>, Vec<GtAttribute>);

impl GtContext {
    pub fn provide_annotation(&mut self, annotation: GtContextAnnotation) {
        self.annotation = Some(annotation);
    }

    pub fn take_annotation_or_default(&mut self) -> GtContextAnnotation {
        self.annotation.take().unwrap_or_default()
    }
}
