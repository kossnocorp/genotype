use miette::highlighters::SyntectHighlighter;
use prelude::internal::*;
use syntect::{highlighting::ThemeSet, parsing::SyntaxDefinition};

mod commands;
pub use commands::*;

mod diagnostic;
pub use diagnostic::*;

pub mod prelude;

#[derive(Parser)]
#[command(name = "gt", version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Builds a Genotype project
    Build(GtBuildCommand),

    /// Initializes a Genotype project
    Init(GtInitCommand),

    /// Manage package versions in genotype.toml
    Version(GtVersionCommand),
}

fn main() -> miette::Result<()> {
    let cli = Cli::parse();

    miette::set_hook(Box::new(|_| {
        Box::new(
            miette::MietteHandlerOpts::new()
                .terminal_links(true)
                .tab_width(2)
                .break_words(true)
                .color(true)
                .with_syntax_highlighting(genotype_highlighter())
                .build(),
        )
    }))?;

    match &cli.command {
        Some(Commands::Build(args)) => GtBuildCommand::run(args),

        Some(Commands::Init(args)) => init_command(args),

        Some(Commands::Version(args)) => version_command(args),

        None => {
            let mut command = Cli::command();
            command.print_help().into_diagnostic()?;
            println!();
            Ok(())
        }
    }
}

fn genotype_highlighter() -> SyntectHighlighter {
    let timeout = std::time::Duration::from_millis(100);
    let bg_theme = termbg::theme(timeout).unwrap_or(termbg::Theme::Light);

    let theme_name = match bg_theme {
        termbg::Theme::Dark => "base16-ocean.dark",
        termbg::Theme::Light => "base16-ocean.light",
    };

    let theme_set = ThemeSet::load_defaults();
    let theme = theme_set.themes[theme_name].clone();

    let syntax = SyntaxDefinition::load_from_str(
        include_str!("../assets/genotype.sublime-syntax"),
        false,
        Some("genotype"),
    )
    .expect("bundled Genotype Sublime syntax should be valid");
    let mut syntax_builder = syntect::parsing::SyntaxSet::load_defaults_nonewlines().into_builder();
    syntax_builder.add(syntax);

    SyntectHighlighter::new(syntax_builder.build(), theme, false)
}
