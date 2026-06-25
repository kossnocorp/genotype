use crate::prelude::internal::*;

/// System compiler.
pub struct GtCompilerSystem;

impl GtCompilerSystem {
    pub fn build(
        base_path: &GtpCwdRelativeOrAbsoluteStringPath,
        config_path: Option<&GtpCwdRelativeOrAbsoluteStringPath>,
    ) -> i32 {
        println!(); // Output padding

        let project = GtpRuntimeSystem::new_and_load_all_modules(base_path, config_path);
        match project {
            Ok(project) => {
                let mut compiler = GtcCompilation::new(&project, &GtcBackendSystem);
                compiler.compile()
            }

            Err(err) => {
                GtcBackendSystem.print_notice(GtNotice::error(err));
                1
            }
        }
    }
}

impl<'a> GtCompiler<GtCompilerSystemInput<'a>, i32> for GtCompilerSystem {
    fn build_once(input: GtCompilerSystemInput) -> Result<i32> {
        let (base_path, config_path) = input;
        Ok(Self::build(base_path, config_path))
    }
}

pub type GtCompilerSystemInput<'a> = (
    &'a GtpCwdRelativeOrAbsoluteStringPath,
    Option<&'a GtpCwdRelativeOrAbsoluteStringPath>,
);
