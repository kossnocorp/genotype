use crate::prelude::internal::*;

/// System compiler.
pub struct GtCompilerSystem;

impl GtCompilerSystem {}

impl<'a> GtCompiler<GtCompilerSystemInput<'a>, i32> for GtCompilerSystem {
    fn build_once(input: GtCompilerSystemInput) -> Result<i32> {
        let (base_path, config_path) = input;

        println!(); // Output padding

        let project = GtpRuntimeSystem::new_and_load_all_modules(base_path, config_path);
        let code = match project {
            Ok(project) => {
                let mut compiler = GtcCompilation::new(&project, &GtcBackendSystem);
                compiler.compile()
            }

            Err(err) => {
                GtcBackendSystem.print_diagnostic(GtDiagnostic::error(err));
                1
            }
        };
        Ok(code)
    }
}

pub type GtCompilerSystemInput<'a> = (
    &'a GtpCwdRelativeOrAbsoluteStringPath,
    Option<&'a GtpCwdRelativeOrAbsoluteStringPath>,
);
