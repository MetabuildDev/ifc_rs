use std::{
    fmt::Display,
    ops::{Deref, DerefMut},
};

use ifc_verify_derive::IfcVerify;

use super::{
    shared::{
        object::{Object, ObjectBuilder},
        product::{Product, ProductBuilder},
        root::{Root, RootBuilder},
        spatial_element::{SpatialElement, SpatialElementBuilder},
        spatial_structure_element::{SpatialStructureElement, SpatialStructureElementBuilder},
    },
    Structure,
};
use crate::{
    ifc_type::{IfcType, IfcVerify},
    parser::{
        comma::Comma, ifc_float::IfcFloat, label::Label, optional::OptionalParameter,
        p_space_or_comment_surrounded, IFCParse, IFCParser,
    },
    IFC,
};

/// The building storey has an elevation and typically represents a (nearly) horizontal aggregation
/// of spaces that are vertically bound.
///
/// A storey is (if specified) associated to a building. A storey may span over several connected
/// storeys. Therefore storey complex provides for a collection of storeys included in a building.
/// A storey can also be decomposed in (horizontical) parts, where each part defines a partial
/// storey. This is defined by the composition type attribute of the supertype
/// IfcSpatialStructureElements which is interpreted as follow:
///
/// COMPLEX: building storey complex
/// ELEMENT: building storey
/// PARTIAL: partial building storey
///
/// EXAMPLE  In split level houses, a storey is split into two or more partial storeys, each
/// with a different elevation. It can be handled by defining a storey, which includes two or
/// more partial storeys with the individual elevations.
///
/// The IfcBuildingStorey is used to build the spatial structure of a building (that serves as the
/// primary project breakdown and is required to be hierarchical). The spatial structure elements
/// are linked together by using the objectified relationship IfcRelAggregates.
///
/// Figure 152 shows the IfcBuildingStorey as part of the s patial structure. It also serves as the
/// spatial container for building and other elements.
///
/// NOTE  Detailed requirements on mandatory element containment and placement structure
/// relationships are given in view definitions and implementer agreements.
///
/// https://standards.buildingsmart.org/IFC/DEV/IFC4_2/FINAL/HTML/link/ifcbuildingstorey.htm
#[derive(IfcVerify)]
pub struct Storey {
    spatial_element_structure: SpatialStructureElement,

    /// Elevation of the base of this storey, relative to the 0,00 internal reference height of the
    /// building. The 0.00 level is given by the absolute above sea level height by the
    /// ElevationOfRefHeight attribute given at IfcBuilding.
    pub elevation: OptionalParameter<IfcFloat>,
}

impl Storey {
    pub fn new<'a>(name: impl Into<Label>) -> Self {
        Self {
            spatial_element_structure: SpatialStructureElement::new(SpatialElement::new(
                Product::new(Object::new(Root::new(name.into()))),
            )),
            elevation: OptionalParameter::omitted(),
        }
    }

    pub fn elevation(mut self, elevation: f64) -> Self {
        self.elevation = IfcFloat(elevation).into();
        self
    }
}

impl RootBuilder for Storey {
    fn root_mut(&mut self) -> &mut Root {
        &mut self.spatial_element_structure
    }
}

impl ObjectBuilder for Storey {
    fn object_mut(&mut self) -> &mut Object {
        &mut self.spatial_element_structure
    }
}

impl ProductBuilder for Storey {
    fn product_mut(&mut self) -> &mut Product {
        &mut self.spatial_element_structure
    }
}

impl SpatialElementBuilder for Storey {
    fn spatial_element_mut(&mut self) -> &mut SpatialElement {
        &mut self.spatial_element_structure
    }
}

impl SpatialStructureElementBuilder for Storey {
    fn spatial_structure_element_mut(&mut self) -> &mut SpatialStructureElement {
        &mut self.spatial_element_structure
    }
}

impl Deref for Storey {
    type Target = SpatialStructureElement;

    fn deref(&self) -> &Self::Target {
        &self.spatial_element_structure
    }
}

impl DerefMut for Storey {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.spatial_element_structure
    }
}

impl IFCParse for Storey {
    fn parse<'a>() -> impl IFCParser<'a, Self> {
        winnow::seq! {
            Self {
                _: p_space_or_comment_surrounded("IFCBUILDINGSTOREY("),

                spatial_element_structure: SpatialStructureElement::parse(),
                _: Comma::parse(),
                elevation: OptionalParameter::parse(),

                _: p_space_or_comment_surrounded(");"),
            }
        }
    }
}

impl Display for Storey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "IFCBUILDINGSTOREY({},{});",
            self.spatial_element_structure, self.elevation,
        )
    }
}

impl IfcType for Storey {}
impl Structure for Storey {}

#[cfg(test)]
mod test {
    use winnow::Parser;

    use super::Storey;
    use crate::parser::IFCParse;

    #[test]
    fn storey_round_trip() {
        let example = "IFCBUILDINGSTOREY('0UQ2T3XlP1QPjq2tKlOviR',#47,'2.UG OKRF',$,$,#127,$,'2.UG OKRF',.ELEMENT.,-7.009);";

        let storey = Storey::parse().parse(example).unwrap();
        let str_storey = storey.to_string();

        assert_eq!(example, str_storey);
    }
}
