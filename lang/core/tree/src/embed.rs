use crate::GtlRenderState;

/// Embed respresenting litral lines of code embeded into language tree.
#[derive(Default, Debug, PartialEq, Clone)]
pub struct GtlEmbed(pub Vec<GtlEmbedLine>);

impl GtlEmbed {
    pub fn render<RenderState: GtlRenderState>(&self, state: RenderState) -> String {
        self.0
            .iter()
            .map(|line| line.render(state))
            .collect::<Vec<_>>()
            .join("\n")
    }
}

impl From<&str> for GtlEmbed {
    fn from(code: &str) -> Self {
        GtlEmbed(
            code.split("\n")
                .into_iter()
                .map(|line| line.into())
                .collect(),
        )
    }
}

impl From<String> for GtlEmbed {
    fn from(code: String) -> Self {
        code.as_str().into()
    }
}

impl From<Vec<&str>> for GtlEmbed {
    fn from(lines: Vec<&str>) -> Self {
        GtlEmbed(lines.into_iter().map(|line| line.into()).collect())
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct GtlEmbedLine(pub String);

impl GtlEmbedLine {
    pub fn render<RenderState: GtlRenderState>(&self, state: RenderState) -> String {
        state.indent_format(&self.0)
    }
}

impl From<&str> for GtlEmbedLine {
    fn from(line: &str) -> Self {
        GtlEmbedLine(line.into())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[derive(Clone, Copy)]
    struct MockState(usize);

    impl GtlRenderState for MockState {
        fn indent_level(&self) -> usize {
            self.0
        }

        fn indent_inc(&self) -> Self {
            MockState(self.0 + 1)
        }
    }

    #[test]
    fn test_embed_render() {
        let embed = GtlEmbed(vec![
            GtlEmbedLine("hello".into()),
            GtlEmbedLine("world".into()),
        ]);
        assert_eq!(
            embed.render(MockState(0)),
            r"hello
world"
        )
    }

    #[test]
    fn test_embed_render_indent() {
        let embed = GtlEmbed(vec![
            GtlEmbedLine("hello".into()),
            GtlEmbedLine("world".into()),
        ]);
        assert_eq!(
            embed.render(MockState(2)),
            r"    hello
    world"
        )
    }
}
