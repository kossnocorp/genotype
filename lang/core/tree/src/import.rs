use std::fmt::Debug;

pub trait GTLImport {}

impl Debug for dyn GTLImport {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("GTLImport")
            // [TODO]
            // .field("imports", &"<omitted>")
            .finish()
    }
}

impl PartialEq for dyn GTLImport {
    fn eq(&self, other: &Self) -> bool {
        // [TODO]
        true
    }
}
