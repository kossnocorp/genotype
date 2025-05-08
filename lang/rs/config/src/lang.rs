use crate::prelude::internal::*;

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct RsConfigLang {
    #[serde(default = "RsConfigLang::default_derive")]
    pub derive: Vec<String>,
}

impl RsConfigLang {
    const DEFAULT_DERIVE: &'static [&str] = &[
        // [TODO] Implement Default for union enums
        // "Default",
        "Debug",
        "Clone",
        "PartialEq",
        // [NOTE] Eq and Hash are disabled by default as float types do not implement them.
        // See: https://github.com/kossnocorp/genotype/issues/9
        // "Eq",
        // "Hash",
    ];

    const FLOAT_UNIMPLEMENTED_DERIVES: &'static [&str] = &["Eq", "Hash", "Ord"];

    pub fn default_derive() -> Vec<String> {
        Self::DEFAULT_DERIVE.iter().map(|s| s.to_string()).collect()
    }

    pub fn needs_ordered_floats(&self) -> bool {
        self.derive
            .iter()
            .any(|d| Self::FLOAT_UNIMPLEMENTED_DERIVES.contains(&d.as_str()))
    }
}

impl Default for RsConfigLang {
    fn default() -> Self {
        Self {
            derive: Self::default_derive(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_ordered_floats() {
        let config = RsConfigLang {
            derive: vec!["Debug".to_string(), "Eq".to_string()],
        };
        assert_eq!(config.needs_ordered_floats(), true);

        let config = RsConfigLang {
            derive: vec!["Debug".to_string(), "Clone".to_string()],
        };
        assert_eq!(config.needs_ordered_floats(), false);
    }
}
