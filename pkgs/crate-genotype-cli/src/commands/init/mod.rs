use crate::prelude::internal::*;
use ratatui::{
    Frame, Terminal, TerminalOptions, Viewport,
    backend::CrosstermBackend,
    buffer::{Buffer, CellDiffOption},
    crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers},
    layout::{Position, Rect},
    style::Stylize,
    text::{Line, Text},
    widgets::Widget,
};
use std::{
    fs::OpenOptions,
    io::{ErrorKind, Write, stdout},
    num::NonZeroU16,
    path::Path,
    process::ExitCode,
};

mod starter;
use starter::*;

const DOCS_URL: &str = "https://genotype-lang.org/docs/toolchain/cli/#gt-init";
const CANVAS_HEIGHT: u16 = 16;

#[derive(Args)]
pub struct GtInitCommand {
    /// Where to initialize the project, by default it will be the current directory.
    #[arg(default_value = ".")]
    path: GtpCwdRelativeOrAbsoluteStringPath,
}

pub fn init_command(args: &GtInitCommand) -> Result<ExitCode> {
    let base: GtpCwdRelativePath = (&args.path).try_into()?;
    let name = project_name(base.as_str());
    let path = base.as_str();
    let path = if path.is_empty() { "./" } else { path };
    let mut app = InitApp::new(detected_targets(base.as_str()), path.into());

    run_wizard(&mut app, |app| {
        let config = app.config(&name)?;

        let backend = GtbSystem::new(&args.path).wrap_err("Failed to create system backend")?;
        block_on(backend.save_config(&base.join_str(GTCONFIG_FILE).into(), &config))?;

        let src = base.join_relative_path(config.src.relative_path());

        create_dir_all(src.as_str())
            .map_err(|_| GtCliError::FailedCreateDir(src.as_str().into()))?;

        let starter = app.starter();
        for (file, contents) in starter.files() {
            let path = src.join_str(file);
            let mut file = OpenOptions::new()
                .write(true)
                .create_new(true)
                .open(path.as_str())
                .map_err(|error| match error.kind() {
                    ErrorKind::AlreadyExists => GtCliError::FileAlreadyExists(path.as_str().into()),
                    _ => GtCliError::FailedWrite(path.as_str().into()),
                })?;
            file.write_all(contents.as_bytes())
                .map_err(|_| GtCliError::FailedWrite(path.as_str().into()))?;
        }

        Ok(())
    })
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum WizardExit {
    Completed,
    Cancelled,
}

fn run_wizard(app: &mut InitApp, finish: impl FnOnce(&InitApp) -> Result<()>) -> Result<ExitCode> {
    ratatui::crossterm::terminal::enable_raw_mode().into_diagnostic()?;
    let terminal = Terminal::with_options(
        CrosstermBackend::new(stdout()),
        TerminalOptions {
            viewport: Viewport::Inline(CANVAS_HEIGHT),
        },
    );
    let mut terminal = match terminal {
        Ok(terminal) => terminal,
        Err(error) => {
            let _ = ratatui::crossterm::terminal::disable_raw_mode();
            return Err(error).into_diagnostic();
        }
    };

    let mut finish = Some(finish);
    let result = (|| -> Result<WizardExit> {
        loop {
            let completed = terminal.draw(|frame| app.render(frame)).into_diagnostic()?;
            app.last_area = completed.buffer.area;
            if let Event::Key(key) = event::read().into_diagnostic()?
                && matches!(key.kind, KeyEventKind::Press | KeyEventKind::Repeat)
                && let Some(exit) = app.handle_key(key)
            {
                if exit == WizardExit::Completed {
                    finish.take().expect("wizard finishes once")(app)?;
                }
                let completed = terminal.draw(|frame| app.render(frame)).into_diagnostic()?;
                app.last_area = completed.buffer.area;
                return Ok(exit);
            }
        }
    })();

    let cursor = if result.is_ok() {
        let area = app.last_area;
        Position::new(
            area.x,
            (area.y + app.final_height()).min(area.bottom().saturating_sub(1)),
        )
    } else {
        let _ = terminal.clear();
        let viewport = terminal.get_frame().area();
        Position::new(viewport.x, viewport.y)
    };
    let _ = terminal.set_cursor_position(cursor);
    let _ = terminal.show_cursor();
    let _ = ratatui::crossterm::terminal::disable_raw_mode();
    result.map(|exit| match exit {
        WizardExit::Completed => ExitCode::SUCCESS,
        WizardExit::Cancelled => ExitCode::FAILURE,
    })
}

struct InitApp {
    step: InitStep,
    targets: UiPromptMultiSelect<AnswerTarget>,
    mode: UiPromptSelect<AnswerPackageMode>,
    packaged: UiPromptMultiSelect<AnswerTarget>,
    paths: Vec<(AnswerTarget, TextPrompt)>,
    starter: UiPromptSelect<StarterResponse>,
    success_path: String,
    last_area: Rect,
}

impl InitApp {
    fn new(target_defaults: Vec<usize>, success_path: String) -> Self {
        Self {
            step: InitStep::Targets,
            targets: UiPromptMultiSelect::new(
                "Choose the languages you want to target",
                AnswerTarget::ALL.to_vec(),
                &target_defaults,
                false,
            ),
            mode: UiPromptSelect::new(
                "How target types should be generated",
                AnswerPackageMode::ALL.to_vec(),
                0,
            ),
            packaged: UiPromptMultiSelect::new(
                "Which targets should generate packages",
                Vec::new(),
                &[],
                true,
            ),
            paths: Vec::new(),
            starter: UiPromptSelect::new(
                "Do you want to generate demo types",
                StarterResponse::ALL.to_vec(),
                1,
            ),
            success_path,
            last_area: Rect::default(),
        }
    }

    fn handle_key(&mut self, key: KeyEvent) -> Option<WizardExit> {
        if key.modifiers.contains(KeyModifiers::CONTROL) && key.code == KeyCode::Char('c') {
            self.step = InitStep::Cancelled;
            return Some(WizardExit::Cancelled);
        }

        match self.step {
            InitStep::Targets => match self.targets.handle_key(key) {
                UiPromptAction::Submit(_) => self.step = InitStep::Mode,

                UiPromptAction::Back => {
                    self.step = InitStep::Cancelled;
                    return Some(WizardExit::Cancelled);
                }

                UiPromptAction::Pending => {}
            },

            InitStep::Mode => match self.mode.handle_key(key) {
                UiPromptAction::Submit(AnswerPackageMode::Generate) => {
                    self.paths.clear();
                    self.step = InitStep::Starter;
                }

                UiPromptAction::Submit(AnswerPackageMode::Integrate) => {
                    self.prepare_paths(self.selected_targets());
                    self.step = InitStep::Path(0);
                }

                UiPromptAction::Submit(AnswerPackageMode::Each) => {
                    let targets = self.selected_targets();
                    if self.packaged.options() != targets {
                        let defaults: Vec<_> = (0..targets.len()).collect();
                        self.packaged = UiPromptMultiSelect::new(
                            "Which targets should generate packages",
                            targets,
                            &defaults,
                            true,
                        );
                    }
                    self.step = InitStep::Packaged;
                }

                UiPromptAction::Back => self.step = InitStep::Targets,

                UiPromptAction::Pending => {}
            },

            InitStep::Packaged => match self.packaged.handle_key(key) {
                UiPromptAction::Submit(packaged) => {
                    let integrated = self
                        .selected_targets()
                        .into_iter()
                        .filter(|target| !packaged.contains(target))
                        .collect();
                    self.prepare_paths(integrated);
                    self.step = if self.paths.is_empty() {
                        InitStep::Starter
                    } else {
                        InitStep::Path(0)
                    };
                }

                UiPromptAction::Back => self.step = InitStep::Mode,

                UiPromptAction::Pending => {}
            },

            InitStep::Path(index) => match self.paths[index].1.handle_key(key) {
                UiPromptAction::Submit(_) if index + 1 < self.paths.len() => {
                    self.step = InitStep::Path(index + 1);
                }

                UiPromptAction::Submit(_) => self.step = InitStep::Starter,

                UiPromptAction::Back if index > 0 => self.step = InitStep::Path(index - 1),

                UiPromptAction::Back if self.mode.selected() == AnswerPackageMode::Each => {
                    self.step = InitStep::Packaged;
                }

                UiPromptAction::Back => self.step = InitStep::Mode,

                UiPromptAction::Pending => {}
            },

            InitStep::Starter => match self.starter.handle_key(key) {
                UiPromptAction::Submit(_) => {
                    self.step = InitStep::Complete;
                    return Some(WizardExit::Completed);
                }

                UiPromptAction::Back if !self.paths.is_empty() => {
                    self.step = InitStep::Path(self.paths.len() - 1);
                }

                UiPromptAction::Back if self.mode.selected() == AnswerPackageMode::Each => {
                    self.step = InitStep::Packaged;
                }

                UiPromptAction::Back => self.step = InitStep::Mode,

                UiPromptAction::Pending => {}
            },

            InitStep::Complete => return Some(WizardExit::Completed),

            InitStep::Cancelled => return Some(WizardExit::Cancelled),
        }
        None
    }

    fn selected_targets(&self) -> Vec<AnswerTarget> {
        self.targets.selected()
    }

    fn prepare_paths(&mut self, targets: Vec<AnswerTarget>) {
        if self
            .paths
            .iter()
            .map(|(target, _)| *target)
            .collect::<Vec<_>>()
            == targets
        {
            return;
        }
        self.paths = targets
            .into_iter()
            .map(|target| {
                (
                    target,
                    TextPrompt::new(format!("Output directory for {target}")),
                )
            })
            .collect();
    }

    fn config(&self, name: &str) -> Result<GtpConfig> {
        let mut config = GtpConfig::default();
        config.name = Some(name.into());
        config.version = Some(Version::parse("0.1.0").into_diagnostic()?);
        config.formatters = vec![];
        for target in self.selected_targets() {
            target.enable(&mut config, name);
        }

        match self.mode.selected() {
            AnswerPackageMode::Generate => {}

            AnswerPackageMode::Integrate => {
                config.package = false;
                config.dist = ".".into();
            }

            AnswerPackageMode::Each => {
                config.dist = ".".into();
                let packaged = self.packaged.selected();
                for target in self.selected_targets() {
                    if !packaged.contains(&target) {
                        target.common_mut(&mut config).package = Some(false);
                    }
                }
            }
        }

        for (target, path) in &self.paths {
            target.common_mut(&mut config).dist = Some(path.value().as_str().into());
        }

        Ok(config)
    }

    fn starter(&self) -> StarterResponse {
        self.starter.selected()
    }

    fn final_height(&self) -> u16 {
        match self.step {
            InitStep::Complete => {
                2 + 3
                    + u16::from(self.mode.selected() == AnswerPackageMode::Each)
                    + self.paths.len() as u16
                    + 6
            }

            InitStep::Cancelled => 1,
            _ => 0,
        }
    }

    fn render(&self, frame: &mut Frame) {
        let area = frame.area();
        if self.step == InitStep::Cancelled {
            frame.render_widget(
                Line::from(vec![
                    "✗".red(),
                    " Cancelled: no files have been created or changed".into(),
                ]),
                Rect::new(area.x, area.y, area.width, 1),
            );
            return;
        }

        let mut y = area.y;
        render_wizard_header(frame, Rect::new(area.x, y, area.width, 1));
        y += 2;

        if self.step != InitStep::Targets {
            self.targets.render_as_answer(&mut y, frame, area);
        }

        if matches!(
            self.step,
            InitStep::Packaged | InitStep::Path(_) | InitStep::Starter | InitStep::Complete
        ) {
            self.mode.render_as_answer(&mut y, frame, area);
        }

        if matches!(
            self.step,
            InitStep::Path(_) | InitStep::Starter | InitStep::Complete
        ) && self.mode.selected() == AnswerPackageMode::Each
        {
            self.packaged.render_as_answer(&mut y, frame, area);
        }

        if let InitStep::Path(active) = self.step {
            for (_, path) in self.paths.iter().take(active) {
                path.render_as_answer(&mut y, frame, area);
            }
        } else if matches!(self.step, InitStep::Starter | InitStep::Complete) {
            for (_, path) in &self.paths {
                path.render_as_answer(&mut y, frame, area);
            }
        }

        if self.step == InitStep::Complete {
            self.starter.render_as_answer(&mut y, frame, area);
            render_success(frame, area, &mut y, &self.success_path);
            return;
        }

        let prompt_area = Rect::new(area.x, y, area.width, area.bottom().saturating_sub(y));
        match self.step {
            InitStep::Targets => self.targets.render(frame, prompt_area),
            InitStep::Mode => self.mode.render(frame, prompt_area),
            InitStep::Packaged => self.packaged.render(frame, prompt_area),
            InitStep::Path(index) => self.paths[index].1.render(frame, prompt_area),
            InitStep::Starter => self.starter.render(frame, prompt_area),
            InitStep::Complete => unreachable!(),
            InitStep::Cancelled => unreachable!(),
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum InitStep {
    Targets,
    Mode,
    Packaged,
    Path(usize),
    Starter,
    Complete,
    Cancelled,
}

fn render_success(frame: &mut Frame, area: Rect, y: &mut u16, path: &str) {
    *y += 1;
    frame.render_widget(
        Line::from(vec![
            "✓".green(),
            " Generated project at ".into(),
            path.cyan(),
        ]),
        Rect::new(area.x, *y, area.width, 1),
    );
    *y += 2;
    frame.render_widget(
        Line::from(vec!["i".yellow(), " To generate your types, run:".into()]),
        Rect::new(area.x, *y, area.width, 1),
    );
    *y += 2;
    frame.render_widget(
        Line::from("    gt build").yellow().bold(),
        Rect::new(area.x, *y, area.width, 1),
    );
    *y += 1;
}

fn render_wizard_header(frame: &mut Frame, area: Rect) {
    let title = "Initializing Genotype project";
    let prefix = " (learn more: ";
    frame.render_widget(Line::from(vec![title.bold(), prefix.dim()]), area);
    let x = area.x + title.len() as u16 + prefix.len() as u16;
    if x >= area.right() {
        return;
    }
    let link_width = (DOCS_URL.len() as u16).min(area.right() - x);
    frame.render_widget(
        &Hyperlink::new(Line::from(DOCS_URL).dim(), DOCS_URL),
        Rect::new(x, area.y, link_width, 1),
    );
    let close_x = x + link_width;
    if close_x < area.right() {
        frame.render_widget(Line::from(")").dim(), Rect::new(close_x, area.y, 1, 1));
    }
}

struct Hyperlink<'a> {
    text: Text<'a>,
    url: &'a str,
}

impl<'a> Hyperlink<'a> {
    fn new(text: impl Into<Text<'a>>, url: &'a str) -> Self {
        Self {
            text: text.into(),
            url,
        }
    }
}

impl Widget for &Hyperlink<'_> {
    fn render(self, area: Rect, buffer: &mut Buffer) {
        (&self.text).render(area, buffer);
        let chars: Vec<_> = self.text.to_string().chars().collect();
        for (index, chunk) in chars.chunks(2).enumerate() {
            let text: String = chunk.iter().collect();
            let hyperlink = format!("\x1b]8;;{}\x07{}\x1b]8;;\x07", self.url, text);
            let x = area.x + index as u16 * 2;
            if x < area.right() {
                let width = NonZeroU16::new(chunk.len() as u16).expect("chunk is not empty");
                buffer[(x, area.y)]
                    .set_symbol(&hyperlink)
                    .set_diff_option(CellDiffOption::ForcedWidth(width));
            }
        }
    }
}

fn project_name(path: &str) -> String {
    Path::new(path)
        .canonicalize()
        .ok()
        .as_deref()
        .unwrap_or(Path::new(path))
        .file_name()
        .map(|value| value.to_string_lossy().into_owned())
        .unwrap_or_else(|| "genotype-project".into())
}

fn detected_targets(path: &str) -> Vec<usize> {
    let mut found = [false; 3];
    let root = if path.is_empty() { "." } else { path };
    let pattern = format!("{}/**/*", glob::Pattern::escape(root));
    if let Ok(paths) = glob::glob(&pattern) {
        for path in paths.flatten() {
            match path.extension().and_then(|value| value.to_str()) {
                Some("ts") => found[0] = true,
                Some("py") => found[1] = true,
                Some("rs") => found[2] = true,
                _ => {}
            }
            if found.iter().all(|found| *found) {
                break;
            }
        }
    }
    found
        .iter()
        .enumerate()
        .filter_map(|(index, found)| found.then_some(index))
        .collect()
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum AnswerTarget {
    TypeScript,
    Python,
    Rust,
}

impl AnswerTarget {
    const ALL: &'static [Self] = &[Self::TypeScript, Self::Python, Self::Rust];

    fn common_mut(self, config: &mut GtpConfig) -> &mut GtpLangConfigCommon {
        match self {
            Self::TypeScript => &mut config.ts.common,
            Self::Python => &mut config.py.common,
            Self::Rust => &mut config.rs.common,
        }
    }

    fn enable(self, config: &mut GtpConfig, name: &str) {
        match self {
            Self::TypeScript => {
                config.ts.common.enabled = true;
                config.ts.common.formatters = vec![];
                config
                    .ts
                    .common
                    .manifest
                    .insert("name".into(), name.to_kebab_case().into());
            }

            Self::Python => {
                config.py.common.enabled = true;
                config.py.lang.manager = PyPackageManager::Uv;
                config.py.common.formatters = vec![];
                config.py.common.manifest.insert(
                    "project".into(),
                    toml::Value::Table(toml::map::Map::from_iter([(
                        "name".into(),
                        name.to_kebab_case().into(),
                    )])),
                );
            }

            Self::Rust => {
                config.rs.common.enabled = true;
                config.rs.common.formatters = vec![];
                config.rs.common.manifest.insert(
                    "package".into(),
                    toml::Value::Table(toml::map::Map::from_iter([
                        ("name".into(), name.to_snake_case().into()),
                        ("edition".into(), RsConfigLang::DEFAULT_EDITION.into()),
                    ])),
                );
            }
        }
    }
}

impl Display for AnswerTarget {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}

ui_select_options!(AnswerPackageMode {
    Generate => "Generate as separate packages",
    Integrate => "Integrate into existing packages",
    Each => "Customize for each target",
});
