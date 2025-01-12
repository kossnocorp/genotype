use std::{
    path::PathBuf,
    sync::{Arc, Mutex},
};

use indexmap::IndexMap;
use miette::Result;

use crate::{
    error::GTWError,
    file::{GTWFile, GTWFiles},
    path::GTWPath,
};

pub struct GTWorkspace {
    files: GTWFiles,
}

impl<'a> GTWorkspace {
    pub fn new() -> GTWorkspace {
        GTWorkspace {
            files: Arc::new(Mutex::new(IndexMap::new())),
        }
    }

    pub fn load_file(&self, path: &String) -> Result<()> {
        let path = GTWPath::new(path)?;

        // Check if the file is already loading
        {
            let files = self.files.lock().map_err(|_| GTWError::FilesLock)?;
            if let Some(GTWFile::Loading) = files.get(&path) {
                // The file is already loading, do nothing
                return Ok(());
            }
        }

        // Load the file content and check if it needs to be reloaded
        let source = GTWFile::read_source(&path);
        // [TODO]

        Ok(())
    }
}
