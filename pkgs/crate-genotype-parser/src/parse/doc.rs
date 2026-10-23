use crate::prelude::internal::*;

pub(crate) fn concat_doc(doc: &GtDoc, pair: Pair<'_, Rule>) -> GtDoc {
    let added_span = pair.as_span();
    let span = (doc.0.0, added_span.end()).into();
    GtDoc(span, format!("{}\n{}", doc.1, pair.as_str()))
}

impl GtParseToken for GtDoc {
    fn parse(pair: Pair<'_, Rule>) -> Self {
        let span = (pair.as_span().start(), pair.as_span().end()).into();
        GtDoc(span, pair.as_str().into())
    }
}
