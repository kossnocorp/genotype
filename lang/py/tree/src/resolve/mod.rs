use crate::PYContext;

pub trait PYContextResolve {
    fn resolve<Context>(self, context: &mut Context) -> Self
    where
        Context: PYContext;
}
