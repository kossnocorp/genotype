use super::name::GTName;

#[derive(Debug, PartialEq, Clone)]
pub enum GTImportName {
    Name(GTName),
    Alias(GTName, GTName),
}
