use crate::prelude::internal::*;

impl<'a> GtlRender<'a> for TsImport {
    type RenderState = TsRenderState;

    type RenderContext = TsRenderContext<'a>;

    fn render(
        &self,
        state: Self::RenderState,
        context: &mut Self::RenderContext,
    ) -> Result<String> {
        let reference = self.reference.render(state, context)?;
        let path = self.dependency.as_path().render(state, context)?;

        Ok(format!(r#"import {reference} from "{path}";"#))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_render_default() {
        assert_snapshot!(
            render_node(Tst::import_default("../path/to/module", "Name")),
            @r#"import Name from "../path/to/module.js";"#
        );
    }

    #[test]
    fn test_render_glob() {
        assert_snapshot!(
            render_node(Tst::import_glob("../path/to/module", "name")),
            @r#"import * as name from "../path/to/module.js";"#
        );
    }

    #[test]
    fn test_render_named() {
        assert_snapshot!(
            render_node(
                Tst::import_named(
                    "../path/to/module",
                    vec![Tst::import_name("Name"), Tst::import_alias("Name", "Alias")],
                ),
            ),
            @r#"import { Name, Name as Alias } from "../path/to/module.js";"#
        );
    }
}
