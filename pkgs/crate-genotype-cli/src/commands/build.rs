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
        Self::build(args);
        Ok(())
    }

    pub fn build(args: &GtBuildCommand) {
        println!(); // Output padding

        let project = GtpRuntimeSystem::new_and_load_all_modules(&args.path, args.config.as_ref());
        match project {
            Ok(project) => {
                let mut compiler = GtCompiler::new(&project, &GtcBackendSystem);
                let exit_code = compiler.compile();

                if exit_code > 0 {
                    std::process::exit(exit_code);
                }
            }

            Err(err) => {
                GtcBackendSystem.print_notice(GtNotice::error(err));
                std::process::exit(1);
            }
        }
    }
}
