use crate::prelude::internal::*;

impl RSTraverse for RSStruct {
    fn traverse(&mut self, visitor: &mut dyn RSVisitor) {
        visitor.visit_struct(self);

        if let Some(doc) = &mut self.doc {
            doc.traverse(visitor);
        }

        for attribute in &mut self.attributes {
            attribute.traverse(visitor);
        }

        self.name.traverse(visitor);

        self.fields.traverse(visitor);
    }
}
