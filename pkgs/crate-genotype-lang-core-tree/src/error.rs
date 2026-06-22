use crate::prelude::internal::*;

pub trait GtlError: StdError + Diagnostic + 'static {
    fn clone_box(&self) -> Box<dyn GtlError>;

    fn boxed(self) -> Box<dyn GtlError>
    where
        Self: Sized,
    {
        Box::new(self)
    }
}

impl Clone for Box<dyn GtlError> {
    fn clone(&self) -> Self {
        self.as_ref().clone_box()
    }
}

impl PartialEq for Box<dyn GtlError> {
    fn eq(&self, other: &Self) -> bool {
        self.to_string() == other.to_string()
    }
}

impl Serialize for dyn GtlError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

impl StdError for Box<dyn GtlError> {}

impl<'a> Borrow<dyn Diagnostic + 'a> for Box<dyn GtlError + 'a> {
    fn borrow(&self) -> &(dyn Diagnostic + 'a) {
        let diagnostic: &(dyn Diagnostic + 'a) = self.as_ref();
        diagnostic
    }
}
