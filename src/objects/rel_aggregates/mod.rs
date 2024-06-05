use std::ops::Deref;

use crate::{
    id::Id,
    parser::{list::IfcList, optional::OptionalParameter},
};

use super::shared::root::Root;

/// The aggregation relationship IfcRelAggregates is a special type of
/// the general composition/decomposition (or whole/part) relationship
/// IfcRelDecomposes. The aggregation relationship can be applied to
/// all subtypes of IfcObjectDefinition.
///
/// https://standards.buildingsmart.org/IFC/DEV/IFC4_2/FINAL/HTML/link/ifcrelaggregates.htm
pub struct RelAggregates {
    root: Root,

    /// The object definition, either an object type or an object
    /// occurrence, that represents the aggregation. It is the whole
    /// within the whole/part relationship.
    pub relating_object: OptionalParameter<Id>,

    /// The object definitions, either object occurrences or object
    /// types, that are being aggregated. They are defined as the
    /// parts in the whole/part relationship. No order is implied
    /// between the parts.
    pub related_objects: IfcList<Id>,
}

impl Deref for RelAggregates {
    type Target = Root;

    fn deref(&self) -> &Self::Target {
        &self.root
    }
}
