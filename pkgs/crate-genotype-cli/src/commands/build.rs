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

pub fn build_command(args: &GtBuildCommand) -> Result<()> {
    let project = GtpRuntimeSystem::new_and_load_all_modules(&args.path, args.config.as_ref())?;

    let mut langs = vec![];

    if project.config.ts_enabled() {
        print_notices(&project.config.ts.health_check());
        let ts = TsProject::generate(&project)?.dist()?;
        langs.push(ts);
    }

    if project.config.python_enabled() {
        print_notices(&project.config.py.health_check());
        let py = PyProject::generate(&project)?.dist()?;
        langs.push(py);
    }

    if project.config.rust_enabled() {
        if project.pkg_config_rs().package_enabled() {
            print_notices(&project.config.rs.health_check());
        }
        let rs = RsProject::generate(&project)?.dist()?;
        langs.push(rs);
    }

    write_dist(&langs).map_err(|_| GtCliError::Write)?;

    println!(
        "{} project to {}",
        "Generated".green().bold(),
        project.config.dist
    );

    Ok(())
}

fn write_dist(projects: &Vec<GtlProjectDist>) -> Result<(), Box<dyn std::error::Error>> {
    for project in projects {
        project.files.iter().try_for_each(|module| {
            let dir = module.path.relative_path().parent().unwrap();
            create_dir_all(dir.to_path(""))?;
            write(module.path.relative_path().to_path(""), &module.source)
        })?;
    }

    Ok(())
}

fn print_notices(notices: &Vec<GtlConfigNotice>) {
    for notice in notices {
        print_notice(notice);
    }
}

fn print_notice(notice: &GtlConfigNotice) {
    match notice.kind {
        GtlConfigNoticeKind::Warning => {
            eprintln!(
                "{label}: {message}",
                label = "Warning".yellow().bold(),
                message = notice.message
            );
        }

        GtlConfigNoticeKind::Info => {
            println!(
                "{label}: {message}",
                label = "Info".blue().bold(),
                message = notice.message
            );
        }
    }
}
