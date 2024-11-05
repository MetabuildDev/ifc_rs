mod deserialize;
mod serialize;

use std::ops::{Deref, DerefMut};

use ifc_rs_verify_derive::IfcVerify;

use super::{
    shared::{
        element::{Element, ElementBuilder},
        object::{Object, ObjectBuilder},
        product::{Product, ProductBuilder},
        root::{Root, RootBuilder},
    },
    Structure, StructureType,
};
use crate::{
    id::TypedId,
    parser::{label::Label, optional::OptionalParameter},
    prelude::*,
};

/// A roof is the covering of the top part of a building, it protects the
/// building against the effects of wheather.
///
/// The IfcRoof shall either be represented:
///
/// * as a roof assembly that aggregates all parts (slabs, rafters and purlins,
///   or other included roofs, such as dormers) with own shape representaion, or
/// * as a single roof without decomposition including all shape representations
///   directly at the roof entity.
///
/// Note: In case of an IfcRoof being the assembly of all parts of the roof the
/// aggregation is handled by the IfcRelAggregates relationship, relating an
/// IfcRoof with the related roof elements, like slabs (represented by IfcSlab),
/// rafters and purlins (represented by IfcBeam), or other included roofs, such
/// as dormers (represented by IfcRoof).
///
/// https://standards.buildingsmart.org/IFC/DEV/IFC4_2/FINAL/HTML/link/ifcroof.htm
#[derive(IfcVerify)]
pub struct Roof {
    #[inherited]
    element: Element,

    /// Predefined generic types for a roof that are specified in an enumeration.
    /// There may be a property set given for the predefined types.
    ///
    /// Note: The PredefinedType shall only be used, if no IfcRoofType is
    /// assigned, providing its own IfcRoofType.PredefinedType.
    pub predefined_type: OptionalParameter<RoofTypeEnum>,
}

impl Roof {
    pub fn new(name: impl Into<Label>) -> Self {
        Self {
            element: Element::new(Product::new(Object::new(Root::new(name.into())))),
            predefined_type: OptionalParameter::omitted(),
        }
    }

    pub fn predefined_type(mut self, predefined_type: impl Into<RoofTypeEnum>) -> Self {
        let id_or = predefined_type.into().into();
        self.predefined_type = id_or;
        self
    }
}

impl RootBuilder for Roof {
    fn root_mut(&mut self) -> &mut Root {
        &mut self.element
    }
}

impl ObjectBuilder for Roof {
    fn object_mut(&mut self) -> &mut Object {
        &mut self.element
    }
}

impl ProductBuilder for Roof {
    fn product_mut(&mut self) -> &mut Product {
        &mut self.element
    }
}

impl ElementBuilder for Roof {
    fn element_mut(&mut self) -> &mut Element {
        &mut self.element
    }
}

impl Deref for Roof {
    type Target = Element;

    fn deref(&self) -> &Self::Target {
        &self.element
    }
}

impl DerefMut for Roof {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.element
    }
}

impl IfcType for Roof {
    fn to_structure(&self) -> Option<&dyn Structure> {
        Some(self)
    }
}
impl Structure for Roof {
    fn structure_type(&self) -> Option<StructureType<'_>> {
        Some(StructureType::Roof(self))
    }
}
impl MaterialRelatable for Roof {}

impl TransformableType for Roof {
    fn shape(&self) -> Option<TypedId<ProductDefinitionShape>> {
        self.representation.custom().cloned()
    }
}
