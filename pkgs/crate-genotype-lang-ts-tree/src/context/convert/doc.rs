use crate::prelude::internal::*;

impl TSConvertContext {
    pub fn provide_doc(&mut self, doc: Option<TSDoc>) {
        self.doc = doc;
    }

    pub fn consume_doc(&mut self) -> Option<TSDoc> {
        self.doc.take()
    }
}
