use std::fs::{create_dir_all, write};

use genotype_config::*;
use genotype_lang_core_project::*;
use genotype_path::*;

pub struct GTWriter;

impl GTWriter {
    pub fn write(
        projects: &Vec<GtlProjectDist>,
        config: &GtConfig,
    ) -> Result<(), Box<dyn std::error::Error>> {
        for project in projects {
            project
                .files
                .iter()
                .map(|module| {
                    // [TODO]
                    // let path = config.root.join(&module.path);
                    let dir = module.path.relative_path().parent().unwrap();
                    create_dir_all(dir.to_path(""))?;
                    write(module.path.relative_path().to_path(""), &module.source)
                })
                .collect::<Result<(), _>>()?;
        }
        Ok(())
    }
}
