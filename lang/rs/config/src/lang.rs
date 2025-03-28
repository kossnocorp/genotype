#[derive(Debug, PartialEq, Clone)]
pub struct RSLangConfig {
    pub derive: Vec<String>,
}

impl RSLangConfig {
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

    pub fn default_derive() -> Vec<String> {
        Self::DEFAULT_DERIVE.iter().map(|s| s.to_string()).collect()
    }
}

impl Default for RSLangConfig {
    fn default() -> Self {
        Self {
            derive: Self::default_derive(),
        }
    }
}
