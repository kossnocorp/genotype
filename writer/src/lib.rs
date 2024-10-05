use std::fs::{create_dir_all, write};

use genotype_lang_core_project::project::GTLangProjectRender;

pub struct GTWriter {
    pub projects: Vec<GTLangProjectRender>,
}

impl GTWriter {
    pub fn new(projects: Vec<GTLangProjectRender>) -> Self {
        Self { projects }
    }

    pub fn write(&self) -> Result<(), Box<dyn std::error::Error>> {
        for project in &self.projects {
            project
                .modules
                .iter()
                .map(|module| {
                    // [TODO]
                    let dir = module.path.parent().unwrap();
                    create_dir_all(dir)?;
                    write(module.path.clone(), module.code.clone())
                })
                .collect::<Result<_, _>>()?;
        }
        Ok(())
    }
}
