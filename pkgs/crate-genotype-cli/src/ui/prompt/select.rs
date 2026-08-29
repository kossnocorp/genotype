use crate::ui::prelude::*;

pub trait UiPromptSelectable {
    const HELP: &'static str;

    type Selectable: Display;

    type Selected;

    fn title(&self) -> &str;

    fn options(&self) -> &[Self::Selectable];

    fn filter(&self) -> &TextArea<'static>;

    fn filter_mut(&mut self) -> &mut TextArea<'static>;

    fn error(&self) -> Option<&str>;

    fn error_mut(&mut self) -> &mut Option<&'static str>;

    fn selected(&self) -> Self::Selected;

    fn selected_to_string(&self) -> String;

    fn cursor(&self) -> &usize;

    fn cursor_mut(&mut self) -> &mut usize;

    fn handle_own_key(&mut self, key: KeyEvent) -> Option<UiPromptAction<Self::Selected>>;

    fn option_widget(&self, index: usize, active: bool) -> impl Widget + '_;

    fn handle_select_key(&mut self, key: KeyEvent) -> UiPromptAction<Self::Selected> {
        if let Some(action) = self.handle_own_key(key) {
            return action;
        }

        match key.code {
            KeyCode::Esc => return UiPromptAction::Back,

            KeyCode::Up => self.move_cursor(-1),

            KeyCode::Down => self.move_cursor(1),

            _ if Self::is_filter_key(key) => {
                self.filter_mut().input(key);
                *self.cursor_mut() = self.filtered().first().copied().unwrap_or(0);
                *self.error_mut() = None;
            }

            _ => {}
        }

        UiPromptAction::Pending
    }

    fn render_option(&self, index: usize, active: bool, y: u16, frame: &mut Frame, area: Rect) {
        frame.render_widget(
            Line::from(if active { ">" } else { " " }).fg(ui_theme_active_fg_if(active)),
            Rect::new(area.x, y, 2, 1),
        );

        frame.render_widget(
            self.option_widget(index, active),
            Rect::new(area.x + 2, y, area.width.saturating_sub(2), 1),
        );
    }

    fn filtered(&self) -> Vec<usize> {
        let query = self.filter().lines()[0].as_str().to_lowercase();
        self.options()
            .iter()
            .enumerate()
            .filter_map(|(index, option)| {
                option
                    .to_string()
                    .to_lowercase()
                    .contains(&query)
                    .then_some(index)
            })
            .collect()
    }

    fn is_filter_key(key: KeyEvent) -> bool {
        matches!(
            key.code,
            KeyCode::Char(_) | KeyCode::Backspace | KeyCode::Delete
        ) && !key.modifiers.contains(KeyModifiers::CONTROL)
    }

    fn move_cursor(&mut self, delta: isize) {
        let filtered = self.filtered();
        if filtered.is_empty() {
            return;
        }

        let position = filtered
            .iter()
            .position(|index| index == self.cursor())
            .unwrap_or(0);

        let next = (position as isize + delta).rem_euclid(filtered.len() as isize) as usize;
        *self.cursor_mut() = filtered[next];
    }

    fn current_filtered(&self) -> Option<usize> {
        let filtered = self.filtered();
        let cursor = self.cursor();
        filtered
            .iter()
            .copied()
            .find(|index| index == cursor)
            .or_else(|| filtered.first().copied())
    }
}

impl<Type: UiPromptSelectable> UiPrompt for Type {
    const HELP: &'static str = Type::HELP;

    type Value = Type::Selected;

    fn title(&self) -> &str {
        Type::title(self)
    }

    fn error(&self) -> Option<&str> {
        Type::error(self)
    }

    fn value(&self) -> Self::Value {
        self.selected()
    }

    fn value_to_string(&self) -> String {
        self.selected_to_string()
    }

    fn handle_key(&mut self, key: KeyEvent) -> UiPromptAction<Self::Value> {
        Type::handle_select_key(self, key)
    }

    fn render_filter(&self, x: u16, frame: &mut Frame, area: Rect) {
        frame.render_widget(
            self.filter(),
            Rect::new(x, area.y, area.right().saturating_sub(x), 1),
        );
    }

    fn render_input(&self, y: &mut u16, frame: &mut Frame, area: Rect) {
        for index in self.filtered() {
            let active = index == *self.cursor();

            self.render_option(index, active, *y, frame, area);

            *y += 1;
        }
    }
}

macro_rules! ui_select_options {
    ($name:ident { $($variant:ident => $label:expr),+ $(,)? }) => {
        #[derive(Clone, Copy, Debug, PartialEq)]
        pub enum $name {
            $($variant),+
        }

        impl $name {
            pub const ALL: &'static [Self] = &[$(Self::$variant),+];
        }

        impl Display for $name {
            fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
                write!(f, "{}", match self {
                    $(Self::$variant => $label),+
                })
            }
        }
    };
}

pub(crate) use ui_select_options;

pub struct UiPromptSelect<Type> {
    title: &'static str,
    options: Vec<Type>,
    cursor: usize,
    filter: TextArea<'static>,
    error: Option<&'static str>,
}

impl<Selectable: Copy + Display> UiPromptSelectable for UiPromptSelect<Selectable> {
    const HELP: &str = "↑↓ to move, enter to select, esc to go back, type to filter";

    type Selectable = Selectable;

    type Selected = Selectable;

    fn title(&self) -> &str {
        self.title
    }

    fn filter(&self) -> &TextArea<'static> {
        &self.filter
    }

    fn filter_mut(&mut self) -> &mut TextArea<'static> {
        &mut self.filter
    }

    fn options(&self) -> &[Self::Selectable] {
        &self.options
    }

    fn error(&self) -> Option<&str> {
        self.error
    }

    fn error_mut(&mut self) -> &mut Option<&'static str> {
        &mut self.error
    }

    fn selected(&self) -> Self::Selected {
        self.options[self.cursor]
    }

    fn selected_to_string(&self) -> String {
        self.selected().to_string()
    }

    fn cursor(&self) -> &usize {
        &self.cursor
    }

    fn cursor_mut(&mut self) -> &mut usize {
        &mut self.cursor
    }

    fn handle_own_key(&mut self, key: KeyEvent) -> Option<UiPromptAction<Self::Selected>> {
        match key.code {
            KeyCode::Enter => {
                if let Some(index) = self.current_filtered() {
                    self.cursor = index;
                    self.error = None;
                    return Some(UiPromptAction::Submit(self.options[index]));
                }

                self.error = Some("No matching options");

                Some(UiPromptAction::Pending)
            }

            _ => None,
        }
    }

    fn option_widget(&self, index: usize, active: bool) -> impl Widget + '_ {
        Line::from(vec![
            self.options[index]
                .to_string()
                .fg(ui_theme_active_fg_if(active)),
        ])
    }
}

impl<Value: Copy + Display> UiPromptSelect<Value> {
    pub fn new(title: &'static str, options: Vec<Value>, cursor: usize) -> Self {
        Self {
            title,
            options,
            cursor,
            filter: UiInputText::widget(),
            error: None,
        }
    }
}

pub struct UiPromptMultiSelect<T> {
    title: &'static str,
    options: Vec<T>,
    selected: Vec<bool>,
    cursor: usize,
    filter: TextArea<'static>,
    error: Option<&'static str>,
    allow_empty: bool,
}

impl<Selectable: Display + Copy> UiPromptSelectable for UiPromptMultiSelect<Selectable> {
    const HELP: &str = "↑↓ to move, space to select one, → to all, ← to none, enter to submit, esc to go back, type to filter";

    type Selectable = Selectable;

    type Selected = Vec<Selectable>;

    fn title(&self) -> &str {
        self.title
    }

    fn filter(&self) -> &TextArea<'static> {
        &self.filter
    }

    fn filter_mut(&mut self) -> &mut TextArea<'static> {
        &mut self.filter
    }

    fn options(&self) -> &[Self::Selectable] {
        &self.options
    }

    fn error(&self) -> Option<&str> {
        self.error
    }

    fn error_mut(&mut self) -> &mut Option<&'static str> {
        &mut self.error
    }

    fn selected(&self) -> Self::Selected {
        self.options
            .iter()
            .zip(&self.selected)
            .filter_map(|(option, selected)| selected.then_some(*option))
            .collect()
    }

    fn selected_to_string(&self) -> String {
        self.selected()
            .iter()
            .map(|option| option.to_string())
            .collect::<Vec<_>>()
            .join(", ")
    }

    fn cursor(&self) -> &usize {
        &self.cursor
    }

    fn cursor_mut(&mut self) -> &mut usize {
        &mut self.cursor
    }

    fn handle_own_key(&mut self, key: KeyEvent) -> Option<UiPromptAction<Vec<Selectable>>> {
        match key.code {
            KeyCode::Char(' ') => {
                if let Some(index) = self.current_filtered() {
                    self.selected[index] = !self.selected[index];
                    self.error = None;
                } else {
                    self.error = Some("No matching options");
                }
            }

            KeyCode::Right => {
                self.selected.fill(true);
                self.error = None;
            }

            KeyCode::Left => {
                self.selected.fill(false);
                self.error = None;
            }

            KeyCode::Enter => {
                if self.current_filtered().is_none() {
                    self.error = Some("No matching options");
                } else if self.allow_empty || self.selected.iter().any(|selected| *selected) {
                    return Some(UiPromptAction::Submit(self.selected()));
                } else {
                    self.error = Some("Please select at least one option");
                }
            }

            _ => return None,
        }

        Some(UiPromptAction::Pending)
    }

    fn option_widget(&self, index: usize, active: bool) -> impl Widget + '_ {
        Checkbox::new(self.options[index].to_string(), self.selected[index])
            .checked_symbol("[x] ")
            .unchecked_symbol("[ ] ")
            .style(ui_theme_active_fg_if(active))
            .checkbox_style(ui_theme_selected_fg_if(self.selected[index]))
    }
}

impl<Value: Copy + Display + PartialEq> UiPromptMultiSelect<Value> {
    pub fn new(
        title: &'static str,
        options: Vec<Value>,
        defaults: &[usize],
        allow_empty: bool,
    ) -> Self {
        let mut selected = vec![false; options.len()];
        for index in defaults {
            if let Some(value) = selected.get_mut(*index) {
                *value = true;
            }
        }

        Self {
            title,
            options,
            selected,
            cursor: 0,
            filter: UiInputText::widget(),
            error: None,
            allow_empty,
        }
    }
}
