#[derive(Debug, PartialEq, Clone)]
pub enum GTImportName {
    Name(String),
    Alias(String, String),
}
