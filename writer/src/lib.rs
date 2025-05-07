use std::fs::{create_dir_all, write};

use genotype_config::GtConfig;
use genotype_lang_core_project::*;

pub struct GTWriter;

impl GTWriter {
    pub fn write(
        projects: &Vec<GtlProjectOut>,
        config: &GtConfig,
    ) -> Result<(), Box<dyn std::error::Error>> {
        for project in projects {
            project
                .files
                .iter()
                .map(|module| {
                    // [TODO]
                    let path = config.file_path(&module.path);
                    let dir = path.parent().unwrap();
                    create_dir_all(dir)?;
                    write(path, &module.source)
                })
                .collect::<Result<(), _>>()?;
        }
        Ok(())
    }
}
