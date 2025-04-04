pub trait GtjSchemaConvert<Node> {
    fn to_schema(&self) -> Node;
}
