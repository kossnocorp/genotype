use super::descriptor::Descriptor;

#[derive(Debug, PartialEq)]
pub struct Union {
    pub descriptors: Vec<Descriptor>,
}
