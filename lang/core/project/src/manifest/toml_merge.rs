use crate::prelude::internal::*;

pub trait TomlMerge {
    fn merge(&self, other: &Self) -> Result<Self>
    where
        Self: Sized;
}

impl TomlMerge for DocumentMut {
    fn merge(&self, other: &Self) -> Result<Self>
    where
        Self: Sized,
    {
        let merged = self.clone();
        for (key, value) in other.iter() {
            //     if let Some(existing_value) = merged.get_mut(key) {
            //         if let Some(existing_table) = existing_value.as_table_mut() {
            //             if let Some(other_table) = value.as_table() {
            //                 existing_table.merge(other_table)?;
            //             }
            //         } else {
            //             *existing_value = value.clone();
            //         }
            //     } else {
            //         merged.insert(key.clone(), value.clone());
            //     }
        }
        Ok(merged)
    }
}
