use crate::prelude::internal::*;

/// System compiler.
pub struct GtcSystem {
    state: GtCompilerState,
    backend: GtbSystem,
}

impl GtcSystem {}

impl GtCompiler<GtcSystemProps<'_>> for GtcSystem {
    fn new(props: GtcSystemProps) -> Result<Self> {
        let (base_path, config_path) = props;
        let config_path = config_path
            .map(|path| {
                path.try_into()
                    .wrap_err_with(|| format!("Failed to normalize config path `{path}`"))
            })
            .transpose()?;

        let backend =
            GtbSystem::new(base_path).wrap_err("Failed to create system project backend")?;

        Ok(Self {
            state: GtCompilerState::New { config_path },
            backend,
        })
    }

    fn state(&self) -> &GtCompilerState {
        &self.state
    }

    fn state_mut(&mut self) -> &mut GtCompilerState {
        &mut self.state
    }

    fn backend(&self) -> &impl GtBackend {
        &self.backend
    }
}

pub type GtcSystemProps<'a> = (
    &'a GtpCwdRelativeOrAbsoluteStringPath,
    Option<&'a GtpCwdRelativeOrAbsoluteStringPath>,
);
