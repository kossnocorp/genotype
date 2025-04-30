use crate::prelude::internal::*;

impl PYTraverse for PYEmbedDefinition {
    fn traverse(&mut self, visitor: &mut dyn PYVisitor) {
        visitor.visit_embed_definition(self);
        self.name.traverse(visitor);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_traverse() {
        let mut visitor = PYMockVisitor::new();
        let name = PYIdentifier("Name".into());
        let mut embed = PYEmbedDefinition {
            name: name.clone(),
            embed: Default::default(),
        };
        embed.traverse(&mut visitor);
        assert_eq!(
            visitor.visited,
            vec![
                PYMockVisited::EmbedDefinition(embed),
                PYMockVisited::Identifier(name)
            ]
        );
    }
}
