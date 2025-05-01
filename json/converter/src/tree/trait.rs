use genotype_parser::GTNamingContextName;

use super::GtjTreeConvertContext;

pub trait GtjTreeConvert<Node> {
    fn to_tree_with_context(&self, context: &mut GtjTreeConvertContext) -> Node;

    fn to_tree(&self) -> Node {
        self.to_tree_with_context(&mut Default::default())
    }

    fn to_named_tree_with_context(
        &self,
        name: String,
        context: &mut GtjTreeConvertContext,
    ) -> Node {
        context.enter_name_context(GTNamingContextName::Identifier(name), |context| {
            self.to_tree_with_context(context)
        })
    }

    fn to_named_tree(&self, name: String) -> Node {
        self.to_named_tree_with_context(name, &mut Default::default())
    }
}
