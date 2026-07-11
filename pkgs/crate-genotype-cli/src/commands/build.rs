use crate::prelude::internal::*;

#[derive(Args)]
pub struct GtBuildCommand {
    /// What to build.
    #[arg(default_value = ".")]
    path: GtpCwdRelativeOrAbsoluteStringPath,

    /// Path to genotype config file
    #[arg(long)]
    config: Option<GtpCwdRelativeOrAbsoluteStringPath>,
}

impl GtBuildCommand {
    pub fn run(args: &GtBuildCommand) -> Result<()> {
        let meta = block_on(GtcSystem::compile_once((&args.path, args.config.as_ref())))?;
        std::process::exit(meta.exit_code);
    }
}
