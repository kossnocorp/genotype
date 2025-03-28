use crate::RS_KEYWORDS;

pub struct RSNaming;

impl RSNaming {
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
        assert_eq!(RSNaming::render("foo"), "foo");
        assert_eq!(RSNaming::render("type"), "r#type");
        assert_eq!(RSNaming::render("r#type"), "r#type");
    }
}
