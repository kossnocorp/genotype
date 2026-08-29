use crate::ui::prelude::*;

pub struct UiInputText;

impl UiInputText {
    pub fn widget() -> TextArea<'static> {
        let mut input = TextArea::default();
        input.set_cursor_line_style(Style::default());
        input
    }
}
