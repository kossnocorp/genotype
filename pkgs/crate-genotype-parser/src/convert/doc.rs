use crate::prelude::internal::*;

pub(crate) fn concat_doc(doc: &GtDoc, token: GtToken<'_>) -> GtDoc {
    let added_span = token.span;
    let span = (doc.0.0, added_span.1).into();
    GtDoc(span, format!("{}\n{}", doc.1, token.as_str()))
}

impl GtParseToken for GtDoc {
    fn parse(token: GtToken<'_>) -> Self {
        let span = token.span;
        GtDoc(span, token.as_str().into())
    }
}
