use crate::indent::GTIndent;

pub trait GTRender {
    fn render(&self, indent: &GTIndent) -> String;
}
