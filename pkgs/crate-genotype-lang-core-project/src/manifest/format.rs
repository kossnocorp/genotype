use crate::prelude::internal::*;

#[derive(Debug, PartialEq, Clone, Serialize)]
pub enum GtlManifestFormat {
    Toml,
    Json,
}

impl Display for GtlManifestFormat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            GtlManifestFormat::Toml => write!(f, "TOML"),
            GtlManifestFormat::Json => write!(f, "JSON"),
        }
    }
}
