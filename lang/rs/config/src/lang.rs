#[derive(Debug, PartialEq, Clone)]
pub struct RSLangConfig {
    pub derive: Vec<String>,
}

impl Default for RSLangConfig {
    fn default() -> Self {
        Self {
            derive: vec![
                "Default".into(),
                "Debug".into(),
                "Clone".into(),
                "PartialEq".into(),
                "Eq".into(),
                "Hash".into(),
                "Serialize".into(),
                "Deserialize".into(),
            ],
        }
    }
}
