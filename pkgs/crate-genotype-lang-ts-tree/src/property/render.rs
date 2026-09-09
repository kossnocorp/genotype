use crate::prelude::internal::*;

impl<'context> GtlRender<'context, TsRenderTypes> for TsProperty {
    fn render(
        &self,
        state: TsRenderState,
        context: &mut TsRenderContext,
    ) -> TsRenderResult<String> {
        let name = self.name.render(state, context)?;

        let str = match context.mode() {
            TsMode::Effect => {
                let mut descriptor = self.descriptor.render(state, context)?;
                let refs = self.descriptor.scan_references();
                if refs.has_forward || refs.has_self_recursive {
                    let type_code = context.with_mode(TsMode::Types, |context| {
                        self.descriptor.render(state, context)
                    })?;
                    descriptor =
                        format!("Schema.suspend((): Schema.Codec<{type_code}> => {descriptor})");
                }
                if !self.required {
                    descriptor = format!("Schema.optionalKey({descriptor})");
                }
                format!("{}{name}: {descriptor}", state.indent_str())
            }
            TsMode::Zod => {
                let mut descriptor = self.descriptor.render(state, context)?;
                if !self.required {
                    descriptor.push_str(".optional()");
                }

                let refs_scan = self.descriptor.scan_references();
                if refs_scan.has_forward || refs_scan.has_self_recursive {
                    format!(
                        "{}get {name}() {{\n{}  return {descriptor}\n{}}}",
                        state.indent_str(),
                        state.indent_str(),
                        state.indent_str(),
                    )
                } else {
                    format!("{}{name}: {descriptor}", state.indent_str())
                }
            }
            TsMode::Types => {
                let descriptor = self.descriptor.render(state, context)?;
                format!(
                    "{}{name}{}: {descriptor}",
                    state.indent_str(),
                    if self.required { "" } else { "?" },
                )
            }
        };

        TsDoc::with_doc(&self.doc, state, context, str, false)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_render_basic() {
        assert_snapshot!(
            render_node(Tst::property("name", Tst::primitive_string())),
            @"name: string"
        );

        assert_snapshot!(
            render_node(Tst::property("name", Tst::reference("Name"))),
            @"name: Name"
        );
    }

    #[test]
    fn test_render_indent() {
        assert_snapshot!(
            Tst::property("name", Tst::primitive_string())
            .render(
                TsRenderState::default().indent_inc(),
                &mut Default::default()
            )
            .unwrap(),
            @"  name: string"
        );
    }

    #[test]
    fn test_render_optional() {
        assert_snapshot!(
            render_node(Tst::property_optional("name", Tst::primitive_string())),
            @"name?: string"
        );
    }

    #[test]
    fn test_render_doc() {
        assert_snapshot!(
            render_node(
                assign!(
                    Tst::property("name", Tst::primitive_string()),
                    doc = Tst::some_doc("Hello, world!"),
                ),
            ),
            @"
        /** Hello, world! */
        name: string
        "
        );
    }

    #[test]
    fn test_render_zod_basic() {
        assert_snapshot!(
            render_node_with(
                Tst::property("name", Tst::primitive_string()),
                &mut Tst::render_context_zod()
            ),
            @"name: z.string()"
        );
    }

    #[test]
    fn test_render_zod_optional() {
        assert_snapshot!(
            render_node_with(
                Tst::property_optional("name", Tst::primitive_string()),
                &mut Tst::render_context_zod()
            ),
            @"name: z.string().optional()"
        );

        assert_snapshot!(
            render_node_with(
                Tst::property_optional("name", Tst::union(vec_into![
                    Tst::primitive_string(),
                    Tst::primitive_undefined(),
                ])),
                &mut Tst::render_context_zod()
            ),
            @"name: z.union([z.string(), z.undefined()]).optional()"
        );
    }

    #[test]
    fn test_render_zod_forward_reference() {
        assert_snapshot!(
            render_node_with(
                Tst::property("next", Tst::reference_forward("Node")),
                &mut Tst::render_context_zod()
            ),
            @"
        get next() {
          return Node
        }
        "
        );

        assert_snapshot!(
            render_node_with(
                Tst::property("next", Tst::tuple(vec_into![
                    Tst::primitive_string(),
                    Tst::reference_forward("Node"),
                ])),
                &mut Tst::render_context_zod()
            ),
            @r"
        get next() {
          return z.tuple([z.string(), Node])
        }
        "
        );
    }

    #[test]
    fn test_render_zod_forward_reference_optional() {
        assert_snapshot!(
            render_node_with(
                Tst::property_optional("next", Tst::reference_forward("Node")),
                &mut Tst::render_context_zod()
            ),
            @"
        get next() {
          return Node.optional()
        }
        "
        );

        assert_snapshot!(
            render_node_with(
                Tst::property_optional("next", Tst::tuple(vec_into![
                    Tst::primitive_string(),
                    Tst::reference_forward("Node"),
                ])),
                &mut Tst::render_context_zod()
            ),
            @"
        get next() {
          return z.tuple([z.string(), Node]).optional()
        }
        "
        );
    }

    #[test]
    fn test_render_zod_self_recursive_reference() {
        assert_snapshot!(
            render_node_with(
                Tst::property("next", Tst::reference_self_recursive("Node")),
                &mut Tst::render_context_zod()
            ),
            @"
        get next() {
          return Node
        }
        "
        );

        assert_snapshot!(
            render_node_with(
                Tst::property("next", Tst::tuple(vec_into![
                    Tst::primitive_string(),
                    Tst::reference_self_recursive("Node"),
                ])),
                &mut Tst::render_context_zod()
            ),
            @r"
        get next() {
          return z.tuple([z.string(), Node])
        }
        "
        );
    }

    #[test]
    fn test_render_zod_self_recursive_reference_optional() {
        assert_snapshot!(
            render_node_with(
                Tst::property_optional("next", Tst::reference_self_recursive("Node")),
                &mut Tst::render_context_zod()
            ),
            @"
        get next() {
          return Node.optional()
        }
        "
        );

        assert_snapshot!(
            render_node_with(
                Tst::property_optional("next", Tst::tuple(vec_into![
                    Tst::primitive_string(),
                    Tst::reference_self_recursive("Node"),
                ])),
                &mut Tst::render_context_zod()
            ),
            @"
        get next() {
          return z.tuple([z.string(), Node]).optional()
        }
        "
        );
    }

    #[test]
    fn test_render_effect_optional_forward_reference() {
        assert_eq!(
            render_node_with(
                Tst::property_optional("next", Tst::reference_forward("Node")),
                &mut Tst::render_context_effect()
            ),
            "next: Schema.optionalKey(Schema.suspend((): Schema.Codec<Node> => Node))"
        );
    }

    #[test]
    fn test_render_effect_required() {
        assert_snapshot!(
            render_node_with(Tst::property("name", Tst::primitive_string()), &mut Tst::render_context_effect()),
            @r#"name: Schema.String"#
        );
    }

    #[test]
    fn test_render_effect_optional() {
        assert_snapshot!(
            render_node_with(Tst::property_optional("name", Tst::primitive_string()), &mut Tst::render_context_effect()),
            @r#"name: Schema.optionalKey(Schema.String)"#
        );
    }

    #[test]
    fn test_render_effect_self_recursive() {
        assert_snapshot!(
            render_node_with(Tst::property("next", Tst::reference_self_recursive("Node")), &mut Tst::render_context_effect()),
            @r#"next: Schema.suspend((): Schema.Codec<Node> => Node)"#
        );
    }

    #[test]
    fn test_render_effect_nested_forward() {
        assert_snapshot!(
            render_node_with(Tst::property("children", Tst::array(Tst::reference_forward("Node"))), &mut Tst::render_context_effect()),
            @r#"children: Schema.suspend((): Schema.Codec<Array<Node>> => Schema.mutable(Schema.Array(Node)))"#
        );
    }
}
