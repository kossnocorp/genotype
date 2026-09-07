use crate::prelude::internal::*;
use ratatui::{
    Terminal, TerminalOptions, Viewport,
    backend::CrosstermBackend,
    crossterm::{
        event::{self, Event, KeyCode, KeyEventKind, KeyModifiers},
        terminal,
    },
};
use std::{
    fs,
    io::stdout,
    path::{Path, PathBuf},
    process::ExitCode,
};

mod files;
use files::FILES;

#[derive(Args)]
pub struct GtSkillCommand {
    #[command(subcommand)]
    command: SkillCommand,
}

#[derive(Subcommand)]
enum SkillCommand {
    /// Install the bundled Genotype skill for an agent
    Install {
        /// Project directory in which to install the skill
        #[arg(default_value = ".")]
        path: PathBuf,
        /// Select an agent without prompting
        #[arg(long, value_enum)]
        agent: Option<SkillAgent>,
    },
    /// Update existing Genotype skills in known project directories
    Update {
        /// Project directory to search
        #[arg(default_value = ".")]
        path: PathBuf,
    },
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, ValueEnum)]
pub(crate) enum SkillAgent {
    Agents,
    ClaudeCode,
    Codex,
    Opencode,
    Cursor,
    GithubCopilot,
}

impl Display for SkillAgent {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Self::Agents => "Install in .agents",
            Self::ClaudeCode => "Claude Code",
            Self::Codex => "Codex",
            Self::Opencode => "OpenCode",
            Self::Cursor => "Cursor",
            Self::GithubCopilot => "GitHub Copilot",
        })
    }
}

impl SkillAgent {
    pub(crate) fn directory(self) -> &'static str {
        match self {
            Self::Agents | Self::Codex => ".agents/skills/genotype",
            Self::ClaudeCode => ".claude/skills/genotype",
            Self::Opencode => ".opencode/skills/genotype",
            Self::Cursor => ".cursor/skills/genotype",
            Self::GithubCopilot => ".github/skills/genotype",
        }
    }
}

pub(crate) fn skill_agent_prompt() -> UiPromptSelect<SkillAgent> {
    UiPromptSelect::new(
        "Where should the Genotype skill be installed",
        SkillAgent::value_variants().to_vec(),
        0,
    )
}

pub fn skill_command(args: &GtSkillCommand) -> Result<ExitCode> {
    match &args.command {
        SkillCommand::Install { path, agent } => {
            let agent = match agent {
                Some(agent) => *agent,
                None => match prompt_agent()? {
                    Some(agent) => agent,
                    None => return Ok(ExitCode::FAILURE),
                },
            };
            let destination = path.join(agent.directory());
            write_skill(&destination)?;
            println!("Installed Genotype skill at {}", destination.display());
        }
        SkillCommand::Update { path } => {
            let updated = update_skills(path)?;
            if updated.is_empty() {
                return Err(miette::miette!(
                    "No Genotype skill found in {}. Run `gt skill install` first.",
                    path.display()
                ));
            }
            for destination in updated {
                println!("Updated Genotype skill at {}", destination.display());
            }
        }
    }
    Ok(ExitCode::SUCCESS)
}

fn prompt_agent() -> Result<Option<SkillAgent>> {
    let mut prompt = skill_agent_prompt();
    terminal::enable_raw_mode().into_diagnostic()?;
    let result = (|| {
        let mut terminal = Terminal::with_options(
            CrosstermBackend::new(stdout()),
            TerminalOptions {
                viewport: Viewport::Inline(12),
            },
        )
        .into_diagnostic()?;
        let result = (|| {
            loop {
                terminal
                    .draw(|frame| prompt.render(frame, frame.area()))
                    .into_diagnostic()?;
                if let Event::Key(key) = event::read().into_diagnostic()?
                    && matches!(key.kind, KeyEventKind::Press | KeyEventKind::Repeat)
                {
                    if key.modifiers.contains(KeyModifiers::CONTROL)
                        && key.code == KeyCode::Char('c')
                    {
                        return Ok(None);
                    }
                    match prompt.handle_key(key) {
                        UiPromptAction::Submit(agent) => return Ok(Some(agent)),
                        UiPromptAction::Back => return Ok(None),
                        UiPromptAction::Pending => {}
                    }
                }
            }
        })();
        let _ = terminal.clear();
        let _ = terminal.show_cursor();
        result
    })();
    let _ = terminal::disable_raw_mode();
    result
}

pub(crate) fn write_skill(destination: &Path) -> Result<()> {
    fs::create_dir_all(destination)
        .into_diagnostic()
        .wrap_err_with(|| format!("Failed to create {}", destination.display()))?;
    for (name, contents) in FILES {
        fs::write(destination.join(name), contents)
            .into_diagnostic()
            .wrap_err_with(|| format!("Failed to write {}", destination.join(name).display()))?;
    }
    Ok(())
}

fn update_skills(base: &Path) -> Result<Vec<PathBuf>> {
    let mut updated = Vec::new();
    for directory in [
        ".agents/skills/genotype",
        ".claude/skills/genotype",
        ".codex/skills/genotype",
        ".opencode/skills/genotype",
        ".cursor/skills/genotype",
        ".github/skills/genotype",
    ] {
        let destination = base.join(directory);
        if destination.join("SKILL.md").is_file() {
            write_skill(&destination)?;
            updated.push(destination);
        }
    }
    Ok(updated)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn installs_complete_bundle_for_every_agent() {
        let temp = tempfile::tempdir().unwrap();
        for agent in SkillAgent::value_variants() {
            let destination = temp.path().join(agent.directory());
            write_skill(&destination).unwrap();
            for (name, contents) in FILES {
                assert_eq!(
                    fs::read_to_string(destination.join(name)).unwrap(),
                    *contents
                );
            }
        }
    }

    #[test]
    fn update_discovers_existing_skills_without_installing_other_agents() {
        let temp = tempfile::tempdir().unwrap();
        assert!(update_skills(temp.path()).unwrap().is_empty());
        for agent in [SkillAgent::ClaudeCode, SkillAgent::Codex] {
            let destination = temp.path().join(agent.directory());
            write_skill(&destination).unwrap();
            fs::write(destination.join("SKILL.md"), "old skill").unwrap();
            fs::write(destination.join("notes.md"), "user notes").unwrap();
        }
        let updated = update_skills(temp.path()).unwrap();
        assert_eq!(updated.len(), 2);
        for destination in updated {
            assert_eq!(
                fs::read_to_string(destination.join("SKILL.md")).unwrap(),
                FILES[0].1
            );
            assert_eq!(
                fs::read_to_string(destination.join("notes.md")).unwrap(),
                "user notes"
            );
        }
        assert!(!temp.path().join(SkillAgent::Cursor.directory()).exists());
    }
}
