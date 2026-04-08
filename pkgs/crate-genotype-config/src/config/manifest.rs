use crate::prelude::internal::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GtConfigVersionPart {
    Major,
    Minor,
    Patch,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum GtConfigVersionTarget {
    Global,
    Lang(GtConfigLang),
}

pub struct GtConfigSetVersionProps {
    pub version: Version,
    pub ts: Option<Version>,
    pub py: Option<Version>,
    pub rs: Option<Version>,
}

impl Default for GtConfigSetVersionProps {
    fn default() -> Self {
        Self {
            version: Version::new(0, 1, 0),
            ts: None,
            py: None,
            rs: None,
        }
    }
}

type VersionUpdates = HashMap<GtConfigVersionTarget, Version>;

const TS_MANIFEST_VERSION_PATH: &str = "version";
const PY_MANIFEST_VERSION_PATH_POETRY: &str = "tool.poetry.version";
const PY_MANIFEST_VERSION_PATH_UV: &str = "project.version";
const RS_MANIFEST_VERSION_PATH: &str = "package.version";

impl GtConfig {
    pub fn set_manifest_version(&mut self, props: GtConfigSetVersionProps) -> Result<()> {
        let GtConfigSetVersionProps {
            version,
            ts,
            py,
            rs,
        } = props;

        let mut updates: VersionUpdates = HashMap::new();

        if let Some(cur_global_version) = &self.version {
            ensure_can_set_version(cur_global_version, &version)?;
            updates.insert(GtConfigVersionTarget::Global, version.clone());
        }

        if let Some(cur_ts_version) = self.lang_manifest_version(GtConfigLang::Ts)? {
            ensure_can_set_version(&cur_ts_version, &version)?;
            let ts_version = ts.as_ref().unwrap_or(&version);
            updates.insert(
                GtConfigVersionTarget::Lang(GtConfigLang::Ts),
                ts_version.clone(),
            );
        }

        if let Some(cur_py_version) = self.lang_manifest_version(GtConfigLang::Py)? {
            ensure_can_set_version(&cur_py_version, &version)?;
            let py_version = py.as_ref().unwrap_or(&version);
            updates.insert(
                GtConfigVersionTarget::Lang(GtConfigLang::Py),
                py_version.clone(),
            );
        }

        if let Some(cur_rs_version) = self.lang_manifest_version(GtConfigLang::Rs)? {
            ensure_can_set_version(&cur_rs_version, &version)?;
            let rs_version = rs.as_ref().unwrap_or(&version);
            updates.insert(
                GtConfigVersionTarget::Lang(GtConfigLang::Rs),
                rs_version.clone(),
            );
        }

        if updates.is_empty() {
            updates.insert(GtConfigVersionTarget::Global, version.clone());
        }

        self.apply_version_updates(updates)
    }

    pub fn bump_manifest_version(&mut self, part: GtConfigVersionPart) -> Result<()> {
        let mut updates: VersionUpdates = HashMap::new();

        if let Some(cur_global_version) = &self.version {
            let next_version = bump_version(cur_global_version, part);
            updates.insert(GtConfigVersionTarget::Global, next_version.clone());
        }

        if let Some(cur_ts_version) = self.lang_manifest_version(GtConfigLang::Ts)? {
            let next_version = bump_version(&cur_ts_version, part);
            updates.insert(
                GtConfigVersionTarget::Lang(GtConfigLang::Ts),
                next_version.clone(),
            );
        }

        if let Some(cur_py_version) = self.lang_manifest_version(GtConfigLang::Py)? {
            let next_version = bump_version(&cur_py_version, part);
            updates.insert(
                GtConfigVersionTarget::Lang(GtConfigLang::Py),
                next_version.clone(),
            );
        }

        if let Some(cur_rs_version) = self.lang_manifest_version(GtConfigLang::Rs)? {
            let next_version = bump_version(&cur_rs_version, part);
            updates.insert(
                GtConfigVersionTarget::Lang(GtConfigLang::Rs),
                next_version.clone(),
            );
        }

        if updates.is_empty() {
            return Err(GtConfigError::ManifestVersionMissingForBump.into());
        }

        self.apply_version_updates(updates)
    }

    fn apply_version_updates(&mut self, updates: VersionUpdates) -> Result<()> {
        for (target, next_version) in updates.into_iter() {
            match target {
                GtConfigVersionTarget::Global => {
                    self.version = Some(next_version.clone());
                }

                GtConfigVersionTarget::Lang(lang) => {
                    let path = self.target_version_path(lang);

                    let item = self
                        .lang_manifest_mut(lang)
                        .drill_str_mut(path)
                        .into_diagnostic()?;
                    *item = toml::Value::String(next_version.to_string());
                }
            }
        }

        Ok(())
    }

    pub fn lang_manifest_mut(&mut self, lang: GtConfigLang) -> &mut toml::Table {
        match lang {
            GtConfigLang::Ts => &mut self.ts.common.manifest,

            GtConfigLang::Py => &mut self.py.common.manifest,

            GtConfigLang::Rs => &mut self.rs.common.manifest,
        }
    }

    pub fn lang_manifest(&self, lang: GtConfigLang) -> &toml::Table {
        match lang {
            GtConfigLang::Ts => &self.ts.common.manifest,

            GtConfigLang::Py => &self.py.common.manifest,

            GtConfigLang::Rs => &self.rs.common.manifest,
        }
    }

    pub fn lang_manifest_version(&self, lang: GtConfigLang) -> Result<Option<Version>> {
        let manifest = self.lang_manifest(lang);
        let path = self.target_version_path(lang);

        if let Some(version_val) = manifest.get_path(path) {
            let version = parse_target_version_toml_val(version_val)?;
            Ok(Some(version))
        } else {
            Ok(None)
        }
    }
}

fn bump_version(version: &Version, part: GtConfigVersionPart) -> Version {
    let mut next = version.clone();
    match part {
        GtConfigVersionPart::Major => {
            next.major += 1;
            next.minor = 0;
            next.patch = 0;
        }

        GtConfigVersionPart::Minor => {
            next.minor += 1;
            next.patch = 0;
        }

        GtConfigVersionPart::Patch => {
            next.patch += 1;
        }
    }
    next
}

impl GtConfig {
    fn target_version_path(&self, lang: GtConfigLang) -> &'static str {
        match lang {
            GtConfigLang::Ts => TS_MANIFEST_VERSION_PATH,
            GtConfigLang::Py => match self.py.lang.manager {
                PyPackageManager::Poetry => PY_MANIFEST_VERSION_PATH_POETRY,
                PyPackageManager::Uv => PY_MANIFEST_VERSION_PATH_UV,
            },
            GtConfigLang::Rs => RS_MANIFEST_VERSION_PATH,
        }
    }
}

fn ensure_can_set_version(current: &Version, next: &Version) -> Result<()> {
    if next < current {
        return Err(GtConfigError::VersionLower {
            current: current.clone(),
            next: next.clone(),
        })
        .into_diagnostic();
    }
    Ok(())
}

fn parse_target_version_toml_val(target: &toml::Value) -> Result<Version> {
    let value = target
        .as_str()
        .ok_or_else(|| GtConfigError::VersionInvalid(target.to_string()))
        .into_diagnostic()?;

    parse_version_str(value)
}

fn parse_version_str(value: &str) -> Result<Version> {
    let version = Version::parse(value)
        .map_err(|_| GtConfigError::VersionInvalid(value.into()))
        .into_diagnostic()?;

    if !version.pre.is_empty() || !version.build.is_empty() {
        return Err(GtConfigError::VersionInvalid(value.into())).into_diagnostic();
    }

    Ok(version)
}

#[cfg(test)]
mod tests {
    use super::*;
    use genotype_test::*;
    use insta::assert_snapshot;

    #[test]
    fn test_set_global_version() {
        let mut config = GtConfig::from_toml_str(
            r#"
version = "0.1.0"

[ts]
enabled = true

[py]
enabled = true

[rs]
enabled = true
"#,
        )
        .unwrap();

        config
            .set_manifest_version(GtConfigSetVersionProps {
                version: Version::parse("0.2.0").unwrap(),
                ..Default::default()
            })
            .unwrap();

        assert_snapshot!(
            config.to_toml_str_pruned().unwrap(),
            @r#"
        version = "0.2.0"

        [ts]
        enabled = true

        [py]
        enabled = true

        [rs]
        enabled = true
        "#
        );
    }

    #[test]
    fn test_set_lang_versions() {
        let mut config = GtConfig::from_toml_str(
            r#"
[ts]
enabled = true

[ts.manifest]
version = "0.1.0"

[py]
enabled = true

[py.manifest.tool.poetry]
version = "0.1.0"

[rs]
enabled = true

[rs.manifest.package]
version = "0.1.1"
"#,
        )
        .unwrap();

        config
            .set_manifest_version(GtConfigSetVersionProps {
                version: Version::parse("0.2.0").unwrap(),
                ..Default::default()
            })
            .unwrap();

        assert_snapshot!(
            config.to_toml_str_pruned().unwrap(),
            @r#"

        [ts]
        enabled = true

        [ts.manifest]
        version = "0.2.0"

        [py]
        enabled = true

        [py.manifest.tool.poetry]
        version = "0.2.0"

        [rs]
        enabled = true

        [rs.manifest.package]
        version = "0.2.0"
        "#
        );
    }

    #[test]
    fn test_set_py_uv_manifest_version() {
        let mut config = GtConfig::from_toml_str(
            r#"
[py]
enabled = true
manager = "uv"

[py.manifest.project]
version = "0.1.0"
"#,
        )
        .unwrap();

        config
            .set_manifest_version(GtConfigSetVersionProps {
                version: Version::parse("0.2.0").unwrap(),
                ..Default::default()
            })
            .unwrap();

        assert_snapshot!(
            config.to_toml_str_pruned().unwrap(),
            @r#"

        [py]
        manager = "uv"
        enabled = true

        [py.manifest.project]
        version = "0.2.0"
        "#
        );
    }

    #[test]
    fn test_set_multi_versions() {
        let mut config = GtConfig::from_toml_str(
            r#"
version = "0.1.0"

[ts]
enabled = true

[py]
enabled = true

[py.manifest.tool.poetry]
version = "0.1.1"

[rs]
enabled = true

[rs.manifest.package]
version = "0.1.2"
"#,
        )
        .unwrap();

        config
            .set_manifest_version(GtConfigSetVersionProps {
                version: Version::parse("0.2.0").unwrap(),
                ..Default::default()
            })
            .unwrap();

        assert_snapshot!(
            config.to_toml_str_pruned().unwrap(),
            @r#"
        version = "0.2.0"

        [ts]
        enabled = true

        [py]
        enabled = true

        [py.manifest.tool.poetry]
        version = "0.2.0"

        [rs]
        enabled = true

        [rs.manifest.package]
        version = "0.2.0"
        "#
        );
    }

    #[test]
    fn test_set_lower_global_version_err() {
        let mut config = GtConfig::from_toml_str(
            r#"
version = "0.3.0"

[ts]
enabled = true

[ts.manifest]
version = "0.3.0"

[py]
enabled = true
"#,
        )
        .unwrap();

        let err = config
            .set_manifest_version(GtConfigSetVersionProps {
                version: Version::parse("0.2.0").unwrap(),
                ..Default::default()
            })
            .unwrap_err();

        assert_debug_snapshot!(
            err,
            @"
        DiagnosticError(
            VersionLower {
                current: Version {
                    major: 0,
                    minor: 3,
                    patch: 0,
                },
                next: Version {
                    major: 0,
                    minor: 2,
                    patch: 0,
                },
            },
        )
        "
        );
    }

    #[test]
    fn test_set_lower_lang_version_err() {
        let mut config = GtConfig::from_toml_str(
            r#"
version = "0.1.0"

[ts]
enabled = true

[ts.manifest]
version = "0.3.0"

[py]
enabled = true
"#,
        )
        .unwrap();

        let err = config
            .set_manifest_version(GtConfigSetVersionProps {
                version: Version::parse("0.2.0").unwrap(),
                ..Default::default()
            })
            .unwrap_err();

        assert_debug_snapshot!(
            err,
            @"
        DiagnosticError(
            VersionLower {
                current: Version {
                    major: 0,
                    minor: 3,
                    patch: 0,
                },
                next: Version {
                    major: 0,
                    minor: 2,
                    patch: 0,
                },
            },
        )
        "
        );
    }

    #[test]
    fn test_bump_global_version() {
        let mut config = GtConfig::from_toml_str(
            r#"
version = "0.1.0"

[ts]
enabled = true

[py]
enabled = true

[rs]
enabled = true
"#,
        )
        .unwrap();

        config
            .bump_manifest_version(GtConfigVersionPart::Minor)
            .unwrap();

        assert_snapshot!(
            config.to_toml_str_pruned().unwrap(),
            @r#"
        version = "0.2.0"

        [ts]
        enabled = true

        [py]
        enabled = true

        [rs]
        enabled = true
        "#
        );
    }

    #[test]
    fn test_bump_lang_versions() {
        let mut config = GtConfig::from_toml_str(
            r#"
[ts]
enabled = true

[ts.manifest]
version = "0.1.0"

[py]
enabled = true

[py.manifest.tool.poetry]
version = "0.1.0"

[rs]
enabled = true

[rs.manifest.package]
version = "0.1.2"
"#,
        )
        .unwrap();

        config
            .bump_manifest_version(GtConfigVersionPart::Minor)
            .unwrap();

        assert_snapshot!(
            config.to_toml_str_pruned().unwrap(),
            @r#"

        [ts]
        enabled = true

        [ts.manifest]
        version = "0.2.0"

        [py]
        enabled = true

        [py.manifest.tool.poetry]
        version = "0.2.0"

        [rs]
        enabled = true

        [rs.manifest.package]
        version = "0.2.0"
        "#
        );
    }

    #[test]
    fn test_bump_py_uv_manifest_version() {
        let mut config = GtConfig::from_toml_str(
            r#"
[py]
enabled = true
manager = "uv"

[py.manifest.project]
version = "0.1.2"
"#,
        )
        .unwrap();

        config
            .bump_manifest_version(GtConfigVersionPart::Minor)
            .unwrap();

        assert_snapshot!(
            config.to_toml_str_pruned().unwrap(),
            @r#"

        [py]
        manager = "uv"
        enabled = true

        [py.manifest.project]
        version = "0.2.0"
        "#
        );
    }

    #[test]
    fn test_bump_multi_versions() {
        let mut config = GtConfig::from_toml_str(
            r#"
version = "0.1.0"

[ts]
enabled = true

[py]
enabled = true

[py.manifest.tool.poetry]
version = "0.1.1"

[rs]
enabled = true

[rs.manifest.package]
version = "0.1.2"
"#,
        )
        .unwrap();

        config
            .bump_manifest_version(GtConfigVersionPart::Minor)
            .unwrap();

        assert_snapshot!(
            config.to_toml_str_pruned().unwrap(),
            @r#"
        version = "0.2.0"

        [ts]
        enabled = true

        [py]
        enabled = true

        [py.manifest.tool.poetry]
        version = "0.2.0"

        [rs]
        enabled = true

        [rs.manifest.package]
        version = "0.2.0"
        "#
        );
    }

    #[test]
    fn test_bump_missing_versions_err() {
        let mut config = GtConfig::from_toml_str(
            r#"
[ts]
enabled = true

[py]
enabled = true
"#,
        )
        .unwrap();

        let err = config
            .bump_manifest_version(GtConfigVersionPart::Minor)
            .unwrap_err();

        assert_debug_snapshot!(
            err,
            @"ManifestVersionMissingForBump"
        );
    }
}
