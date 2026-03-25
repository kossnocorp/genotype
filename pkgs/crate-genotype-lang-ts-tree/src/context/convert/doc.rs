use crate::prelude::internal::*;

impl TsConvertContext {
    pub fn provide_doc(&mut self, doc: Option<TsDoc>) {
        self.doc = doc;
    }

    pub fn consume_doc(&mut self) -> Option<TsDoc> {
        self.doc.take()
    }
}
