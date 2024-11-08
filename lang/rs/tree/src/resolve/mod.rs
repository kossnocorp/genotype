use crate::RSContext;

pub trait RSContextResolve {
    fn resolve<Context>(self, context: &mut Context) -> Self
    where
        Context: RSContext;
}
