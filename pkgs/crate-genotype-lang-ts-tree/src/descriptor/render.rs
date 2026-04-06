use crate::prelude::internal::*;

impl<'a> GtlRender<'a> for TsDescriptor {
    type RenderState = TsRenderState;

    type RenderContext = TsRenderContext<'a>;

    fn render(
        &self,
        state: Self::RenderState,
        context: &mut Self::RenderContext,
    ) -> Result<String> {
        match self {
            TsDescriptor::Array(array) => array.render(state, context),
            TsDescriptor::InlineImport(import) => import.render(state, context),
            TsDescriptor::Intersection(intersection) => intersection.render(state, context),
            TsDescriptor::Literal(literal) => literal.render(state, context),
            TsDescriptor::Primitive(primitive) => primitive.render(state, context),
            TsDescriptor::Reference(name) => name.render(state, context),
            TsDescriptor::Object(object) => object.render(state, context),
            TsDescriptor::Tuple(tuple) => tuple.render(state, context),
            TsDescriptor::Union(union) => union.render(state, context),
            TsDescriptor::Record(record) => record.render(state, context),
            TsDescriptor::Any(any) => any.render(state, context),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test::*;
    use insta::assert_snapshot;

    #[test]
    fn test_render_array() {
        assert_snapshot!(
            render_node(Tst::descriptor(Tst::array(Tst::primitive_number()))),
            @"Array<number>"
        );
    }

    #[test]
    fn test_render_inline_import() {
        assert_snapshot!(
            render_node(
                Tst::descriptor(Tst::inline_import("../path/to/module", "Name")),
            ),
            @r#"import("../path/to/module.js").Name"#
        );
    }

    #[test]
    fn test_render_intersection() {
        assert_snapshot!(
            render_node(
                Tst::descriptor(Tst::intersection(vec_into![
                    Tst::object(vec![Tst::property("hello", Tst::primitive_string())]),
                    Tst::reference("World"),
                ])),
            ),
            @"
        {
          hello: string
        } & World
        "
        );
    }

    #[test]
    fn test_render_object() {
        assert_snapshot!(
            render_node(
                Tst::descriptor(Tst::object(vec![
                    Tst::property("name", Tst::primitive_string()),
                    Tst::property_optional("age", Tst::primitive_number()),
                ])),
            ),
            @"
        {
          name: string,
          age?: number
        }
        "
        );
    }

    #[test]
    fn test_render_primitive() {
        assert_snapshot!(
            render_node(Tst::descriptor(Tst::primitive_boolean())),
            @"boolean"
        );
        assert_snapshot!(
            render_node(Tst::descriptor(Tst::primitive_string())),
            @"string"
        );
    }

    #[test]
    fn test_render_reference() {
        assert_snapshot!(
            render_node(Tst::descriptor(Tst::reference("Name"))),
            @"Name"
        );
    }

    #[test]
    fn test_render_tuple() {
        assert_snapshot!(
            render_node(
                Tst::descriptor(Tst::tuple(vec_into![
                    Tst::primitive_number(),
                    Tst::primitive_string(),
                ])),
            ),
            @"[number, string]"
        );
    }

    #[test]
    fn test_render_union() {
        assert_snapshot!(
            render_node(
                Tst::descriptor(Tst::union(vec_into![
                    Tst::primitive_string(),
                    Tst::primitive_number(),
                ])),
            ),
            @"string | number"
        );
    }

    #[test]
    fn test_render_record() {
        assert_snapshot!(
            render_node(
                Tst::descriptor(Tst::record(Tst::record_key_string(), Tst::primitive_number())),
            ),
            @"Record<string, number>"
        );
    }

    #[test]
    fn test_render_any() {
        assert_snapshot!(
            render_node(Tst::descriptor(Tst::any())),
            @"any"
        );
    }
}
