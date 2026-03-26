use crate::prelude::internal::*;

#[derive(Args)]
pub struct GtVersionCommand {
    #[command(subcommand)]
    pub command: GtVersionSubcommand,
}

#[derive(Subcommand)]
pub enum GtVersionSubcommand {
    /// Set the project package version
    Set(GtVersionSetCommand),
    /// Bump the project package version
    Bump(GtVersionBumpCommand),
}

#[derive(Args)]
pub struct GtVersionSetCommand {
    /// Global version
    pub version: Version,
    /// TypeScript version
    #[arg(long)]
    pub ts: Option<Version>,
    /// Python version
    #[arg(long)]
    pub py: Option<Version>,
    /// Rust version
    #[arg(long, alias = "rust")]
    pub rs: Option<Version>,
    /// Where to apply the update
    #[arg(default_value = ".")]
    pub path: GtRootPath,
}

#[derive(Args)]
pub struct GtVersionBumpCommand {
    /// Which part to bump
    #[arg(value_enum, default_value = "minor")]
    pub part: GtVersionBumpPart,
    /// Where to apply the update
    #[arg(default_value = ".")]
    pub path: GtRootPath,
}

#[derive(Debug, Clone, Copy, ValueEnum)]
pub enum GtVersionBumpPart {
    Major,
    Minor,
    Patch,
}

pub fn version_command(args: &GtVersionCommand) -> Result<()> {
    let path: PathBuf = match &args.command {
        GtVersionSubcommand::Set(args) => args.path.as_str().into(),
        GtVersionSubcommand::Bump(args) => args.path.as_str().into(),
    };

    let mut config = GtConfig::load(&path)?;

    match &args.command {
        GtVersionSubcommand::Set(args) => config.set_manifest_version(GtConfigSetVersionProps {
            version: args.version.clone(),
            ts: args.ts.clone(),
            py: args.py.clone(),
            rs: args.rs.clone(),
        })?,

        GtVersionSubcommand::Bump(args) => config.bump_manifest_version(match args.part {
            GtVersionBumpPart::Major => GtConfigVersionPart::Major,
            GtVersionBumpPart::Minor => GtConfigVersionPart::Minor,
            GtVersionBumpPart::Patch => GtConfigVersionPart::Patch,
        })?,
    }

    config.save(&path)
}
