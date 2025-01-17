use std::{
    collections::HashSet,
    sync::{Arc, Mutex},
};

use indexmap::IndexMap;
use miette::Result;

use crate::{
    error::GTWError,
    file::{GTWFile, GTWFileSource, GTWFiles},
    path::GTWPath,
};

pub struct GTWorkspace {
    /// Workspace root path. It helps to format relative paths, i.e. to display
    /// errors in an editor, so that it can open the file.
    path: GTWPath,
    /// Workspace files map. It associated the absolute path with the file.
    /// Note that the file doesn't necessarily is inside the workspace path,
    /// as editors can open files outside the workspace.
    files: GTWFiles,
}

impl<'a> GTWorkspace {
    pub fn try_new(path_str: &String) -> Result<GTWorkspace> {
        let path = GTWPath::try_new(path_str, None)?;

        Ok(GTWorkspace {
            path,
            files: Arc::new(Mutex::new(IndexMap::new())),
        })
    }

    pub fn load_file(
        &self,
        path_str: &String,
        processing: Arc<Mutex<HashSet<GTWPath>>>,
    ) -> Result<()> {
        let path = GTWPath::try_new(path_str, Some(&self.path))?;

        // Check if the file is already processing
        {
            let mut processing = processing.lock().map_err(|_| GTWError::FilesLock)?;
            if processing.contains(&path) {
                return Ok(());
            }
            processing.insert(path.clone());
        }

        // Load the source
        let source = GTWFileSource::read(&path)?;

        // Check if the file exists and needs no reloading
        {
            let files = self.files.lock().map_err(|_| GTWError::FilesLock)?;
            if let Some(file) = files.get(&path) {
                if file.same_hash(&source) {
                    return Ok(());
                }
            }
        }

        // Load the file
        let file = GTWFile::load(&path, source)?;

        // Update the files map
        {
            let mut files = self.files.lock().map_err(|_| GTWError::FilesLock)?;
            files.insert(path, file);
        }

        Ok(())
    }
}
