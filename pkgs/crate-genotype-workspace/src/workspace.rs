use std::sync::{Arc, Mutex};

use indexmap::{IndexMap, IndexSet};
use miette::Result;

use crate::{
    error::GtwError,
    file::{GtwFile, GtwFileSource, GtwFiles},
    path::GtwPath,
};

pub struct GtWorkspace {
    /// Workspace root path. It helps to format relative paths, i.e. to display
    /// errors in an editor, so that it can open the file.
    path: GtwPath,
    /// Workspace files map. It associated the absolute path with the file.
    /// Note that the file doesn't necessarily is inside the workspace path,
    /// as editors can open files outside the workspace.
    files: GtwFiles,
}

impl GtWorkspace {
    pub fn try_new(path_str: &String) -> Result<GtWorkspace> {
        let path = GtwPath::try_new(path_str, None)?;

        Ok(GtWorkspace {
            path,
            files: Arc::new(Mutex::new(IndexMap::new())),
        })
    }

    pub fn load_file(
        &self,
        path_str: &String,
        processing: Arc<Mutex<IndexSet<GtwPath>>>,
    ) -> Result<()> {
        let path = GtwPath::try_new(path_str, Some(&self.path))?;

        // Check if the file is already processing
        {
            let mut processing = processing.lock().map_err(|_| GtwError::FilesLock)?;
            if processing.contains(&path) {
                return Ok(());
            }
            processing.insert(path.clone());
        }

        // Load the source
        let source = GtwFileSource::read(&path)?;

        // Check if the file exists and needs no reloading
        {
            let files = self.files.lock().map_err(|_| GtwError::FilesLock)?;
            if let Some(file) = files.get(&path)
                && file.same_hash(&source)
            {
                return Ok(());
            }
        }

        // Load the file
        let file = GtwFile::load(&path, source)?;

        // Update the files map
        {
            let mut files = self.files.lock().map_err(|_| GtwError::FilesLock)?;
            files.insert(path, file);
        }

        Ok(())
    }
}
