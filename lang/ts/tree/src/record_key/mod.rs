mod convert;
mod render;

#[derive(Debug, PartialEq, Clone)]
pub enum TSRecordKey {
    Number,
    String,
    Boolean,
}
