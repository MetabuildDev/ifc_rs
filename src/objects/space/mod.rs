mod deserialize;
mod serialize;

use std::ops::{Deref, DerefMut};

use ifc_verify_derive::IfcVerify;

use super::shared::{
    object::{Object, ObjectBuilder},
    product::{Product, ProductBuilder},
    root::{Root, RootBuilder},
};
use crate::{
    ifc_type::{IfcType, IfcVerify},
    parser::{ifc_float::IfcFloat, label::Label, optional::OptionalParameter},
    prelude::{
        SpaceTypeEnum, SpatialElement, SpatialElementBuilder, SpatialStructureElement,
        SpatialStructureElementBuilder, Structure,
    },
    IFC,
};

/// A space represents an area or volume bounded actually or theoretically.
/// Spaces are areas or volumes that provide for certain functions within a
/// building.
///
/// A space is associated to a building storey (or in case of exterior spaces
/// to a site). A space may span over several connected spaces. Therefore a
/// space group provides for a collection of spaces included in a storey. A
/// space can also be decomposed in parts, where each part defines a partial
/// space. This is defined by the CompositionType attribute of the supertype
/// IfcSpatialStructureElement which is interpreted as follow:
///
/// * COMPLEX = space group
/// * ELEMENT = space
/// * PARTIAL = partial space
///
/// Note: View definitions and implementation agreements may restrict spaces
/// with CompositionType=ELEMENT to be non-overlapping.
///
/// The IfcSpace is used to build the spatial structure of a building (that
/// serves as the primary project breakdown and is required to be hierarchical).
/// The spatial structure elements are linked together by using the objectified
/// relationship IfcRelAggregates.
///
/// https://standards.buildingsmart.org/IFC/DEV/IFC4_2/FINAL/HTML/schema/ifcproductextension/lexical/ifcspace.htm
#[derive(IfcVerify)]
pub struct Space {
    #[inherited]
    spatial_element_structure: SpatialStructureElement,

    /// Predefined generic types for a space that are specified in an
    /// enumeration. There might be property sets defined specifically for
    /// each predefined type.
    pub predefined_type: OptionalParameter<SpaceTypeEnum>,

    /// Level of flooring of this space; the average shall be taken, if the
    /// space ground surface is sloping or if there are level differences
    /// within this space.
    pub elevation_with_flooring: OptionalParameter<IfcFloat>,
}

impl Space {
    pub fn new(name: impl Into<Label>) -> Self {
        Self {
            spatial_element_structure: SpatialStructureElement::new(SpatialElement::new(
                Product::new(Object::new(Root::new(name.into()))),
            )),
            predefined_type: OptionalParameter::omitted(),
            elevation_with_flooring: OptionalParameter::omitted(),
        }
    }

    pub fn predefined_type(mut self, predefined_type: impl Into<SpaceTypeEnum>) -> Self {
        let id_or = predefined_type.into();
        self.predefined_type = id_or.into();
        self
    }

    pub fn elevation_with_flooring(mut self, elevation_with_flooring: f64) -> Self {
        self.elevation_with_flooring = IfcFloat(elevation_with_flooring).into();
        self
    }
}

impl RootBuilder for Space {
    fn root_mut(&mut self) -> &mut Root {
        &mut self.spatial_element_structure
    }
}

impl ObjectBuilder for Space {
    fn object_mut(&mut self) -> &mut Object {
        &mut self.spatial_element_structure
    }
}

impl ProductBuilder for Space {
    fn product_mut(&mut self) -> &mut Product {
        &mut self.spatial_element_structure
    }
}

impl SpatialElementBuilder for Space {
    fn spatial_element_mut(&mut self) -> &mut SpatialElement {
        &mut self.spatial_element_structure
    }
}

impl SpatialStructureElementBuilder for Space {
    fn spatial_structure_element_mut(&mut self) -> &mut SpatialStructureElement {
        &mut self.spatial_element_structure
    }
}

impl Deref for Space {
    type Target = SpatialStructureElement;

    fn deref(&self) -> &Self::Target {
        &self.spatial_element_structure
    }
}

impl DerefMut for Space {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.spatial_element_structure
    }
}

impl IfcType for Space {
    fn to_structure(&self) -> Option<&dyn Structure> {
        Some(self)
    }
}
impl Structure for Space {}

#[cfg(test)]
pub mod test {
    use winnow::Parser;

    use crate::objects::space::Space;
    use crate::parser::IFCParse;

    #[test]
    fn space_round_trip() {
        let example = "IFCSPACE('2xcLNHPon6VO6wB_n0EQLF',#12,'002',$,$,#24381,#24456,'Besprechungsraum II',.ELEMENT.,$,$);";

        let space = Space::parse().parse(example).unwrap();
        let str_space = space.to_string();

        assert_eq!(example, str_space);
    }
}
