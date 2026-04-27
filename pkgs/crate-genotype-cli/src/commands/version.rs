use crate::prelude::internal::*;

#[derive(Args)]
pub struct GtVersionCommand {
    #[command(subcommand)]
    pub command: GtVersionSubcommand,
}

impl GtVersionCommand {
    pub fn path(&self) -> &GtpCwdRelativeOrAbsoluteStringPath {
        match &self.command {
            GtVersionSubcommand::Set(args) => &args.path,
            GtVersionSubcommand::Bump(args) => &args.path,
        }
    }
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
    pub path: GtpCwdRelativeOrAbsoluteStringPath,
}

#[derive(Args)]
pub struct GtVersionBumpCommand {
    /// Which part to bump
    #[arg(value_enum, default_value = "minor")]
    pub part: GtVersionBumpPart,
    /// Where to apply the update
    #[arg(default_value = ".")]
    pub path: GtpCwdRelativeOrAbsoluteStringPath,
}

#[derive(Debug, Clone, Copy, ValueEnum)]
pub enum GtVersionBumpPart {
    Major,
    Minor,
    Patch,
}

pub fn version_command(args: &GtVersionCommand) -> Result<()> {
    let path = args.path();
    let project_runtime =
        GtpRuntimeSystem::new(&path).wrap_err("failed to create system project runtime")?;

    let mut project = project_runtime
        .create_project(None)
        .wrap_err("failed to create project")?;

    match &args.command {
        GtVersionSubcommand::Set(args) => {
            project
                .config
                .set_manifest_version(GtpConfigSetVersionProps {
                    version: args.version.clone(),
                    ts: args.ts.clone(),
                    py: args.py.clone(),
                    rs: args.rs.clone(),
                })?
        }

        GtVersionSubcommand::Bump(args) => {
            project.config.bump_manifest_version(match args.part {
                GtVersionBumpPart::Major => GtpConfigVersionPart::Major,
                GtVersionBumpPart::Minor => GtpConfigVersionPart::Minor,
                GtVersionBumpPart::Patch => GtpConfigVersionPart::Patch,
            })?
        }
    }

    project.config.save(&path)
}
