pub mod deserialize;
mod serialize;

use std::collections::BTreeMap;

use crate::{id::Id, ifc_type::IfcType};

/// CRITICAL: split up the index map into a proper struct with fields which hold Hashmaps mapping
/// indices to one specific type instead of an enum
#[derive(Default)]
pub struct DataMap(BTreeMap<Id, Box<dyn IfcType>>);

impl DataMap {
    pub fn insert_new<T: IfcType + 'static>(&mut self, value: T) -> Id {
        let new_id = self
            .0
            .keys()
            .max()
            .cloned()
            .map(|id| Id(id.0 + 1))
            .unwrap_or(Id(1));

        self.insert(new_id, value);

        new_id
    }

    pub fn insert<T: IfcType + 'static>(&mut self, id: Id, value: T) -> Option<Box<dyn IfcType>> {
        self.0.insert(id, Box::new(value))
    }

    pub fn insert_if_not_exists<T: Default + IfcType + 'static>(&mut self, id: Id) {
        if !self.contains(id) {
            self.insert(id, T::default());
        }
    }

    pub fn remove(&mut self, id: Id) -> Option<Box<dyn IfcType>> {
        self.0.remove(&id)
    }

    pub fn get<T: IfcType>(&self, id: Id) -> &T {
        self.get_untyped(id).downcast_ref().unwrap()
    }

    pub fn get_untyped(&self, id: Id) -> &Box<dyn IfcType> {
        self.0.get(&id).unwrap()
    }

    pub fn get_mut<T: IfcType>(&mut self, id: Id) -> &mut T {
        self.0
            .get_mut(&id)
            .map(|any| any.downcast_mut())
            .flatten()
            .unwrap()
    }

    pub fn contains(&self, id: Id) -> bool {
        self.0.contains_key(&id)
    }

    pub fn find_all_of_type<T: IfcType>(&self) -> impl Iterator<Item = &T> {
        self.0
            .values()
            .filter_map(|ifc_type| ifc_type.downcast_ref())
    }
}

impl<I> From<I> for DataMap
where
    I: IntoIterator<Item = (Id, Box<dyn IfcType>)>,
{
    fn from(value: I) -> Self {
        Self(value.into_iter().collect())
    }
}
