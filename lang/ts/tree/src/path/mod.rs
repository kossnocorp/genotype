mod render;

#[derive(Debug, PartialEq, Clone)]
pub enum TSPath {
    Unresolved(String),
    Resolved(String),
}
