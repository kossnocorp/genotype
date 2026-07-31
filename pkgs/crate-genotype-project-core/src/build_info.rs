use crate::prelude::internal::*;

impl GtpBuildInfo {
    pub fn parse(source: &str) -> Result<Self> {
        toml::from_str(source).map_err(|err| miette!(err))
    }

    pub fn to_toml_string(&self) -> Result<String> {
        toml::to_string_pretty(self).map_err(|err| miette!(err))
    }
}

#[allow(clippy::derivable_impls)]
impl Default for GtpBuildInfoSrc {
    fn default() -> Self {
        Self {
            config_hash: GtpSourceCodeHash::default(),
            modules: BTreeMap::new(),
        }
    }
}

impl Eq for GtpBuildInfoSrc {}

impl GtpBuildInfoPath {
    // pub fn as_relative_path()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn hash(source: &str) -> GtpSourceCodeHash {
        GtpSourceCodeHash::new(source)
    }

    #[test]
    fn test_roundtrip() {
        let build_info = GtpBuildInfo {
            src: GtpBuildInfoSrc {
                config_hash: hash("config"),
                modules: BTreeMap::from([(
                    GtpBuildInfoPath("src/model.type".into()),
                    GtpBuildInfoSrcModule {
                        id: "model".into(),
                        hash: hash("model"),
                        deps: vec!["shared".into(), "user".into()],
                    },
                )]),
            },
            dist: GtpBuildInfoDist {
                ts: Some(BTreeMap::from([
                    (
                        GtpBuildInfoPath("dist/ts/model.ts".into()),
                        GtpBuildInfoDistFile {
                            hash: hash("generated model"),
                            src_id: Some("model".into()),
                        },
                    ),
                    (
                        GtpBuildInfoPath("dist/ts/index.ts".into()),
                        GtpBuildInfoDistFile {
                            hash: hash("index"),
                            src_id: None,
                        },
                    ),
                ])),
                rs: None,
                py: None,
            },
        };
        let source = build_info.to_toml_string().unwrap();

        assert_eq!(GtpBuildInfo::parse(&source).unwrap(), build_info);
    }
}
