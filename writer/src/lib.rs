use std::fs::{create_dir_all, write};

use genotype_config::GTConfig;
use genotype_lang_core_project::project::GTLangProjectRender;

pub struct GTWriter;

impl GTWriter {
    pub fn write(
        projects: &Vec<GTLangProjectRender>,
        config: &GTConfig,
    ) -> Result<(), Box<dyn std::error::Error>> {
        for project in projects {
            project
                .modules
                .iter()
                .map(|module| {
                    // [TODO]
                    let path = config.source_path(&module.path);
                    let dir = path.parent().unwrap();
                    create_dir_all(dir)?;
                    write(path, &module.code)
                })
                .collect::<Result<_, _>>()?;
        }
        Ok(())
    }
}
