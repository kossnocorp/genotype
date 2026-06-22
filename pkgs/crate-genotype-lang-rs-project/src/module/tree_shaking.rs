use crate::prelude::internal::*;

impl RsProjectModule {
    pub(crate) fn tree_shake_modules(
        modules: &mut IndexMap<GtpModulePath, GtlProjectModuleState<RsProjectModule>>,
    ) {
        // Remove all unused imports
        *modules = mem::take(modules)
            .into_iter()
            .map(|(module_path, module_state)| {
                let new_state = match module_state {
                    state @ GtlProjectModuleState::ConvertError(_)
                    | state @ GtlProjectModuleState::ResolveError(_)
                    | state @ GtlProjectModuleState::RenderError(_) => {
                        // Do nothing as it is in the final state.
                        state
                    }

                    GtlProjectModuleState::Rendered(inner) => inner.to_resolve_error_state(),

                    GtlProjectModuleState::Converted(inner) => {
                        let mut resolved_inner = inner.to_resolved();
                        Self::tree_shake_module(&mut resolved_inner);
                        resolved_inner.into()
                    }

                    GtlProjectModuleState::Resolved(mut inner) => {
                        Self::tree_shake_module(&mut inner);
                        inner.into()
                    }
                };
                (module_path, new_state)
            })
            .collect();
    }

    fn tree_shake_module(inner_state: &mut GtlProjectModuleResolved<RsProjectModule>) {
        // TODO: We can also remove unused definitions here, but we need to be careful with the
        // order of the definitions and the imports. For now, we only remove unused imports.
        inner_state
            .resolved_module
            .module
            .imports
            .retain(|r#use| match &r#use.dependency {
                // Only process local dependencies for now
                RsDependencyIdent::Local(path) => {
                    inner_state
                        .resolved_module
                        .project_resolve
                        .definitions
                        .iter()
                        .any(|(definition_id, resolve)| {
                            // If there are any references to the definition, we keep the use.
                            definition_id.0 == path.0 && !resolve.references.is_empty()
                        })
                }

                // TODO: Process external dependencies too.
                _ => true,
            });
    }
}
