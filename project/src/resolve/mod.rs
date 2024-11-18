use std::collections::HashMap;

use genotype_parser::*;
use miette::Result;

use crate::{error::GTProjectError, GTProjectModuleParse};

pub mod visitor;

pub struct GTProjectResolve {
    pub definitions: HashMap<GTModuleId, Vec<GTDefinitionId>>,
    pub imports: HashMap<GTModuleId, Vec<GTDefinitionId>>,
    pub paths: HashMap<GTModuleId, HashMap<String, GTModuleId>>,
}

impl GTProjectResolve {
    pub fn new() -> GTProjectResolve {
        GTProjectResolve {
            definitions: HashMap::new(),
            imports: HashMap::new(),
            paths: HashMap::new(),
        }
    }
}

impl TryFrom<&Vec<GTProjectModuleParse>> for GTProjectResolve {
    type Error = miette::Error;

    fn try_from(modules_parse: &Vec<GTProjectModuleParse>) -> Result<GTProjectResolve> {
        let mut definitions: HashMap<GTModuleId, Vec<GTDefinitionId>> = HashMap::new();
        for module in modules_parse {
            let module_definitions = module
                .1
                .resolve
                .exports
                .iter()
                .map(|export| GTDefinitionId(module.1.module.id.clone(), export.1.clone()))
                .collect();
            definitions.insert(module.1.module.id.clone(), module_definitions);
        }

        let mut paths: HashMap<GTModuleId, HashMap<String, GTModuleId>> = HashMap::new();
        let mut imports: HashMap<GTModuleId, Vec<GTDefinitionId>> = HashMap::new();
        for module in modules_parse {
            let mut module_paths: HashMap<String, GTModuleId> = HashMap::new();

            for local_path in module.1.resolve.deps.iter() {
                let module_id = if let Some(module_id) = module_paths.get(local_path.as_str()) {
                    // It's already resolved
                    module_id.clone()
                } else {
                    // Get the project module path from the local path
                    let path = module.0.resolve(local_path).map_err(|_| {
                        GTProjectError::CannotResolve(local_path.as_str().to_owned())
                    })?;
                    // [TODO] Get rid of paths in favor of ids and path -> id resolve?
                    let id = GTModuleId(path.as_id().as_str().to_owned());
                    module_paths.insert(local_path.as_str().into(), id.clone());
                    id
                };

                if let Some(definitions) = definitions.get(&module_id) {
                    imports
                        .entry(module.1.module.id.clone())
                        .or_insert(Default::default())
                        .extend(definitions.clone());
                }
            }

            paths.insert(module.1.module.id.clone(), module_paths);
        }

        Ok(GTProjectResolve {
            definitions,
            paths,
            imports,
        })
    }
}
