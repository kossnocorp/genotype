use crate::ui::prelude::*;

mod text;
pub use text::*;

mod select;
pub use select::*;

pub enum UiPromptAction<Type> {
    Pending,
    Submit(Type),
    Back,
}

pub trait UiPrompt {
    const HELP: &'static str;

    type Value;

    fn title(&self) -> &str;

    fn error(&self) -> Option<&str>;

    fn value(&self) -> Self::Value;

    fn value_to_string(&self) -> String;

    fn handle_key(&mut self, key: KeyEvent) -> UiPromptAction<Self::Value>;

    fn render_filter(&self, _x: u16, _frame: &mut Frame, _area: Rect) {}

    fn render_input(&self, y: &mut u16, frame: &mut Frame, area: Rect);

    fn render(&self, frame: &mut Frame, area: Rect) {
        let mut y = self.render_title(frame, area);

        self.render_input(&mut y, frame, area);

        self.render_help(frame, area, y);
    }

    fn render_title(&self, frame: &mut Frame, area: Rect) -> u16 {
        let title = self.title();
        let prefix = format!("? {title}: ");

        frame.render_widget(
            Line::from(vec!["?".green().bold(), format!(" {title}: ").into()]),
            Rect::new(area.x, area.y, area.width, 1),
        );

        let x = area
            .x
            .saturating_add(prefix.chars().count() as u16)
            .min(area.right());

        self.render_filter(x, frame, area);

        area.y + 1
    }

    fn render_help(&self, frame: &mut Frame, area: Rect, y: u16) {
        if let Some(error) = self.error() {
            frame.render_widget(
                Line::from(error).fg(Color::Red),
                Rect::new(area.x, y, area.width, 1),
            );

            frame.render_widget(
                Line::from(format!("[{help}]", help = Self::HELP)).dim(),
                Rect::new(area.x, y + 1, area.width, 1),
            );
        } else {
            frame.render_widget(
                Line::from(format!("[{help}]", help = Self::HELP)).dim(),
                Rect::new(area.x, y, area.width, 1),
            );
        }
    }

    fn render_as_answer(&self, y: &mut u16, frame: &mut Frame, area: Rect) {
        frame.render_widget(
            Line::from(vec![
                ">".fg(ui_theme_highlight()),
                format!(" {title}: ", title = self.title()).into(),
                self.value_to_string().fg(ui_theme_active_fg()).bold(),
            ]),
            Rect::new(area.x, *y, area.width.saturating_sub(2), 1),
        );
        *y += 1;
    }
}
