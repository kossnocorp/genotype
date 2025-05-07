use std::path::PathBuf;

pub trait GtlConfigOut: Default {
    const DEFAULT_OUT: &'static str;

    fn as_path<'a>(&'a self) -> &'a PathBuf;

    fn default_out() -> PathBuf {
        PathBuf::from(Self::DEFAULT_OUT)
    }
}
