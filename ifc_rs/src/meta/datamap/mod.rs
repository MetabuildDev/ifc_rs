mod deserialize;
mod serialize;

use std::{any::TypeId, collections::BTreeMap};

use crate::{
    id::{Id, TypedId},
    prelude::*,
};

/// CRITICAL: split up the index map into a proper struct with fields which hold Hashmaps mapping
/// indices to one specific type instead of an enum
#[derive(Default)]
pub struct DataMap(pub(crate) BTreeMap<Id, Box<dyn IfcType>>);

impl DataMap {
    pub fn insert_new<T: IfcType + 'static>(&mut self, value: T) -> TypedId<T> {
        let new_id = self
            .0
            .keys()
            .max()
            .cloned()
            .map(|id| Id(id.0 + 1))
            .unwrap_or(Id(1));

        self.insert(new_id, value);

        TypedId::new(new_id)
    }

    pub fn insert<T: IfcType + 'static>(
        &mut self,
        id: impl Into<Id>,
        value: T,
    ) -> Option<Box<dyn IfcType>> {
        self.0.insert(id.into(), Box::new(value))
    }

    pub fn insert_if_not_exists<T: Default + IfcType + 'static>(&mut self, id: impl Into<Id>) {
        let id = id.into();
        if !self.contains(&id) {
            self.insert(id, T::default());
        }
    }

    pub fn remove(&mut self, id: impl Into<Id>) -> Option<Box<dyn IfcType>> {
        self.0.remove(&id.into())
    }

    pub fn get<T: IfcType>(&self, typed_id: TypedId<T>) -> &T {
        self.get_untyped(typed_id.id()).downcast_ref().unwrap()
    }

    pub fn get_untyped(&self, id: impl Into<Id>) -> &dyn IfcType {
        &**self.0.get(&id.into()).unwrap()
    }

    pub fn get_mut<T: IfcType>(&mut self, typed_id: TypedId<T>) -> &mut T {
        self.0
            .get_mut(&typed_id.id())
            .and_then(|any| any.downcast_mut())
            .unwrap()
    }

    pub fn contains(&self, id: &Id) -> bool {
        self.0.contains_key(id)
    }

    pub fn find_all_of_type<T: IfcType>(&self) -> impl Iterator<Item = (TypedId<T>, &T)> {
        self.0
            .iter()
            .filter_map(|(id, ifc_type)| ifc_type.downcast_ref().map(|t| (TypedId::new(*id), t)))
    }

    pub fn id_of<T: IfcType>(&self) -> impl Iterator<Item = TypedId<T>> + '_ {
        self.0
            .iter()
            .filter(|&(_, ifc_type)| (ifc_type.type_id() == TypeId::of::<T>()))
            .map(|(id, _)| TypedId::new(*id))
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
