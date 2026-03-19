use crate::prelude::internal::*;

impl RSTraverse for RSEnumVariant {
    fn traverse(&mut self, visitor: &mut dyn RSVisitor) {
        if let Some(doc) = &mut self.doc {
            doc.traverse(visitor);
        }

        for attribute in &mut self.attributes {
            attribute.traverse(visitor);
        }

        self.name.traverse(visitor);

        if let Some(descriptor) = &mut self.descriptor {
            descriptor.traverse(visitor);
        }
    }
}
