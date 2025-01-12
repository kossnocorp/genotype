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
    files: GTWFiles,
}

impl<'a> GTWorkspace {
    pub fn new() -> GTWorkspace {
        GTWorkspace {
            files: Arc::new(Mutex::new(IndexMap::new())),
        }
    }

    pub fn load_file(&self, path: &String, processing: Arc<Mutex<HashSet<GTWPath>>>) -> Result<()> {
        let path = GTWPath::new(path)?;

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
        let file = GTWFile::load(&path, &source)?;

        // Update the files map
        {
            let mut files = self.files.lock().map_err(|_| GTWError::FilesLock)?;
            files.insert(path, file);
        }

        Ok(())
    }
}
