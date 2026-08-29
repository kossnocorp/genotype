use crate::ui::prelude::*;

pub fn ui_theme_highlight() -> Color {
    Color::Cyan
}

pub fn ui_theme_highlight_if(active: bool) -> Color {
    if active {
        ui_theme_highlight()
    } else {
        Color::Reset
    }
}

pub fn ui_theme_active_fg() -> Color {
    Color::Blue
}

pub fn ui_theme_active_fg_if(active: bool) -> Color {
    if active {
        ui_theme_active_fg()
    } else {
        Color::Reset
    }
}

pub fn ui_theme_selected_fg() -> Color {
    Color::Green
}
pub fn ui_theme_selected_fg_if(selected: bool) -> Color {
    if selected {
        ui_theme_selected_fg()
    } else {
        Color::Reset
    }
}
