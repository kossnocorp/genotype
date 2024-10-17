use std::sync::{Arc, Mutex};

use genotype_lang_py_tree::PYDefinition;

use crate::{context::PYConvertContext, resolve::PYConvertResolve};

pub fn mock_context() -> (Arc<Mutex<Vec<PYDefinition>>>, PYConvertContext) {
    let hoisted = Arc::new(Mutex::new(vec![]));
    let context = {
        let hoisted = Arc::clone(&hoisted);
        PYConvertContext::new(
            PYConvertResolve::new(),
            Box::new(move |definition| {
                let mut hoisted = hoisted.lock().unwrap();
                hoisted.push(definition);
            }),
        )
    };
    (hoisted, context)
}
