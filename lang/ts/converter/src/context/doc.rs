use genotype_lang_ts_tree::TSDoc;

use super::TSConvertContext;

impl TSConvertContext {
    pub fn provide_doc(&mut self, doc: Option<TSDoc>) {
        self.doc = doc;
    }

    pub fn consume_doc(&mut self) -> Option<TSDoc> {
        self.doc.take()
    }
}
