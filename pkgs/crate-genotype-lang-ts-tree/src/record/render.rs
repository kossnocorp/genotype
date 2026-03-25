use crate::prelude::internal::*;

impl<'a> GtlRender<'a> for TsRecord {
    type RenderState = TsRenderState;

    type RenderContext = TsRenderContext<'a>;

    fn render(
        &self,
        state: Self::RenderState,
        context: &mut Self::RenderContext,
    ) -> Result<String> {
        let key = self.key.render(state, context)?;
        let descriptor = self.descriptor.render(state, context)?;

        Ok(format!("Record<{key}, {descriptor}>"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use insta::assert_snapshot;

    #[test]
    fn test_render() {
        assert_snapshot!(
            TsRecord {
                key: TsRecordKey::Number,
                descriptor: TsDescriptor::Primitive(TsPrimitive::String)
            }
            .render(Default::default(), &mut Default::default())
            .unwrap(),
            @"Record<number, string>"
        );
    }
}
