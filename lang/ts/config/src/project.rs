use std::path::PathBuf;

pub struct TSProjectConfig {
    pub out: PathBuf,
    pub src: PathBuf,
}

impl TSProjectConfig {
    pub fn source_path(&self, path: PathBuf) -> PathBuf {
        self.out.join(self.src.clone()).join(path)
    }
}
