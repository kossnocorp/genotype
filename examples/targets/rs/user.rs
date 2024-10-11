pub struct User {
    pub name: Name,
    pub age: Option<usize>,
}

pub struct Name {
    pub first: String,
    pub last: String,
}
