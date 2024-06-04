pub mod deserialize;
mod serialize;

use std::{any::Any, collections::BTreeMap, fmt::Display, mem::transmute};

use anyhow::{anyhow, Context, Result};
use winnow::{combinator::alt, Parser};

use crate::{geometry::Geometry, id::Id, objects::Objects, units::Units};

pub trait ParsedIfcType: Any + Display {}

/// CRITICAL: split up the index map into a proper struct with fields which hold Hashmaps mapping
/// indices to one specific type instead of an enum
pub struct ParsedMap(BTreeMap<Id, Box<dyn Display>>);

impl ParsedMap {
    pub fn parse_types(mut s: &str) -> Result<Box<dyn Display>> {
        alt((Objects::parse(), Geometry::parse(), Units::parse()))
            .parse_next(&mut s)
            .map_err(|err| anyhow!("content parsing failed: {err:#?}"))
            .with_context(|| format!("content: {s}"))
    }

    pub fn insert<T: Display + 'static>(&mut self, id: Id, value: T) -> Option<T> {
        self.0
            .insert(id, Box::new(value))
            .map(|any| *Self::downcast_unchecked(any))
    }

    pub fn insert_if_not_exists<T: Default + Display + 'static>(&mut self, id: Id) {
        if !self.contains(id) {
            self.insert(id, T::default());
        }
    }

    pub fn remove<T: Display>(&mut self, id: Id) -> Option<T> {
        self.0.remove(&id).map(|any| *Self::downcast_unchecked(any))
    }

    pub fn remove_untyped(&mut self, id: Id) -> Option<Box<dyn Display>> {
        self.0.remove(&id)
    }

    pub fn get<T: Display>(&self, id: Id) -> &T {
        self.0
            .get(&id)
            .map(|any| Self::downcast_ref_unchecked(any))
            .unwrap()
    }

    pub fn get_mut<T: Display>(&mut self, id: Id) -> &mut T {
        self.0
            .get_mut(&id)
            .map(|any| Self::downcast_mut_unchecked(any))
            .unwrap()
    }

    pub fn contains(&self, id: Id) -> bool {
        self.0.contains_key(&id)
    }
}

impl ParsedMap {
    fn downcast_unchecked<T: Display>(boxed: Box<dyn Display>) -> Box<T> {
        unsafe { Box::from_raw(Box::into_raw(boxed) as *mut T) }
    }

    fn downcast_ref_unchecked<T: Display>(boxed_ref: &Box<dyn Display>) -> &T {
        unsafe {
            let ptr_to_ptr: *const *const T =
                transmute(destructure_traitobject::data(boxed_ref as *const _));

            &**ptr_to_ptr
        }
    }

    fn downcast_mut_unchecked<T: Display>(boxed_ref: &mut Box<dyn Display>) -> &mut T {
        unsafe {
            let ptr_to_ptr: *mut *mut T =
                transmute(destructure_traitobject::data(boxed_ref as *mut _));

            &mut **ptr_to_ptr
        }
    }
}

impl From<Vec<(Id, Box<dyn Display>)>> for ParsedMap {
    fn from(value: Vec<(Id, Box<dyn Display>)>) -> Self {
        Self(value.into_iter().collect())
    }
}
