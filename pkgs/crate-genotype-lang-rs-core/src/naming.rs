use crate::RS_KEYWORDS;

pub struct RsNaming;

impl RsNaming {
    pub fn render(name: &str) -> String {
        if RS_KEYWORDS.contains(&name) {
            format!("r#{}", name)
        } else {
            name.to_string()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_render() {
        assert_eq!(RsNaming::render("foo"), "foo");
        assert_eq!(RsNaming::render("type"), "r#type");
        assert_eq!(RsNaming::render("r#type"), "r#type");
    }
}
