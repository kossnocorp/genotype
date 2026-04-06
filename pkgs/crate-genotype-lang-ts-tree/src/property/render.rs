use crate::prelude::internal::*;

impl<'a> GtlRender<'a> for TsProperty {
    type RenderState = TsRenderState;

    type RenderContext = TsRenderContext<'a>;

    fn render(
        &self,
        state: Self::RenderState,
        context: &mut Self::RenderContext,
    ) -> Result<String> {
        let name = self.name.render(state, context)?;

        let str = if context.is_zod_mode() {
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
        } else {
            let descriptor = self.descriptor.render(state, context)?;
            format!(
                "{}{name}{}: {descriptor}",
                state.indent_str(),
                if self.required { "" } else { "?" },
            )
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
}
