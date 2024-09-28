use crate::indent::Indent;

pub trait Node {
    fn render(&self, indent: &Indent) -> String;
}
