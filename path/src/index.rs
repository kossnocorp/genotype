use relative_path::RelativePathBuf;

pub struct GtCwdPath(RelativePathBuf);

impl GtCwdPath {
    pub fn new(path: RelativePathBuf) -> Self {
        Self(path)
    }

    pub fn to_path(&self) -> RelativePathBuf {
        self.0.clone()
    }
}
