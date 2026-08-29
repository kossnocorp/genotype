use crate::ui::prelude::*;

pub struct TextPrompt {
    title: String,
    input: TextArea<'static>,
    error: Option<&'static str>,
}

impl UiPrompt for TextPrompt {
    const HELP: &'static str = "enter to submit, esc to go back";

    type Value = String;

    fn title(&self) -> &str {
        &self.title
    }

    fn error(&self) -> Option<&str> {
        self.error
    }

    fn value(&self) -> Self::Value {
        self.input.lines()[0].clone()
    }

    fn value_to_string(&self) -> String {
        self.value()
    }

    fn handle_key(&mut self, key: KeyEvent) -> UiPromptAction<Self::Value> {
        match key.code {
            KeyCode::Esc => UiPromptAction::Back,

            KeyCode::Enter if self.value().trim().is_empty() => {
                self.error = Some("A value is required");
                UiPromptAction::Pending
            }

            KeyCode::Enter => {
                self.error = None;
                UiPromptAction::Submit(self.value().clone())
            }

            _ => {
                self.input.input(key);
                self.error = None;
                UiPromptAction::Pending
            }
        }
    }

    fn render_input(&self, y: &mut u16, frame: &mut Frame, area: Rect) {
        frame.render_widget(&self.input, Rect::new(area.x, *y, area.width, 1));
        *y += 1;
    }
}

impl TextPrompt {
    pub fn new(title: String) -> Self {
        Self {
            title,
            input: UiInputText::widget(),
            error: None,
        }
    }
}
