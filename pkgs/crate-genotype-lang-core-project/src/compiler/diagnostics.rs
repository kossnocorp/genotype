use crate::prelude::internal::*;

pub fn generate_module_diagnostics<ProjectModule: GtlProjectModule>(
    modules: &GtlProjectModules<ProjectModule>,
) -> Vec<GtDiagnostic> {
    let mut diagnostics = vec![];

    for (_module_path, module_state) in modules {
        match module_state {
            state @ GtlProjectModuleState::ConvertError(_)
            | state @ GtlProjectModuleState::ResolveError(_)
            | state @ GtlProjectModuleState::RenderError(_) => {
                diagnostics.push(GtDiagnostic::error(format_module_error_state_message(
                    state,
                )));
            }

            state @ GtlProjectModuleState::Converted(_)
            | state @ GtlProjectModuleState::Resolved(_) => {
                diagnostics.push(GtDiagnostic::error(format_invalid_module_state_message(
                    state,
                )));
            }

            GtlProjectModuleState::Rendered(_) => {}
        }
    }

    if let Some(render_diagnostic) = generate_render_files_diagnostic(modules) {
        diagnostics.push(render_diagnostic);
    }

    diagnostics
}

fn generate_render_files_diagnostic<ProjectModule: GtlProjectModule>(
    modules: &GtlProjectModules<ProjectModule>,
) -> Option<GtDiagnostic> {
    let mut messages = vec![];

    for module in modules.values() {
        match module {
            state @ GtlProjectModuleState::ConvertError(_)
            | state @ GtlProjectModuleState::ResolveError(_)
            | state @ GtlProjectModuleState::RenderError(_) => {
                messages.push(format_module_error_state_message(state))
            }
            state @ GtlProjectModuleState::Converted(_)
            | state @ GtlProjectModuleState::Resolved(_) => {
                messages.push(format_invalid_module_state_message(state));
            }
            _ => {}
        }
    }

    if messages.is_empty() {
        None
    } else {
        let title = "Some of the modules failed to render".to_string();
        let body = messages
            .into_iter()
            .map(|msg| format!("- {msg}"))
            .collect::<Vec<_>>()
            .join("\n");
        Some(GtDiagnostic::warning((title, body)))
    }
}

fn format_module_error_state_message(
    state: &GtlProjectModuleState<impl GtlProjectModule>,
) -> String {
    let action = state.action();
    let (source_path_str, target_path_str) = format_module_state_paths(state);
    format!("Failed to {action} `{target_path_str}` from `{source_path_str}`")
}

fn format_invalid_module_state_message<ProjectModule: GtlProjectModule>(
    state: &GtlProjectModuleState<ProjectModule>,
) -> String {
    let name = state.name();
    let (source_path_str, target_path_str) = format_module_state_paths(state);
    format!("`{target_path_str}` is in error state '{name}' (from `{source_path_str}`)")
}

fn format_module_state_paths<ProjectModule: GtlProjectModule>(
    module: &GtlProjectModuleState<ProjectModule>,
) -> (String, String) {
    let source_path = format!("{}", module.source_path());
    let target_path = module
        .target_path()
        .map(|p| format!("{p}"))
        .unwrap_or_else(|| "<unresolved>".into());
    (source_path, target_path)
}
