use std::{
    fmt::Display,
    ops::{Deref, DerefMut},
};

use super::{
    address::PostalAddress,
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
    id::{Id, IdOr},
    ifc_type::IfcType,
    parser::{
        comma::Comma, ifc_float::IfcFloat, label::Label, optional::OptionalParameter,
        p_space_or_comment_surrounded, IFCParse, IFCParser,
    },
    IFC,
};

/// A building represents a structure that provides shelter for its occupants
/// or contents and stands in one place. The building is also used to provide
/// a basic element within the spatial structure hierarchy for the components
/// of a building project (together with site, storey, and space).
///
/// https://standards.buildingsmart.org/IFC/DEV/IFC4_2/FINAL/HTML/link/ifcbuilding.htm
pub struct Building {
    spatial_element_structure: SpatialStructureElement,

    /// Elevation above sea level of the reference height used for all storey
    /// elevation measures, equals to height 0.0. It is usually the ground
    /// floor level.
    pub elevation_of_ref_height: OptionalParameter<IfcFloat>,

    /// Elevation above the minimal terrain level around the foot print of
    /// the building, given in elevation above sea level.
    pub elevation_of_terrain: OptionalParameter<IfcFloat>,

    /// Address given to the building for postal purposes.
    pub building_address: OptionalParameter<Id>,
}

impl Building {
    pub fn new<'a>(global_id: impl Into<Label>) -> Self {
        Self {
            spatial_element_structure: SpatialStructureElement::new(SpatialElement::new(
                Product::new(Object::new(Root::new(global_id.into()))),
            )),
            elevation_of_ref_height: OptionalParameter::omitted(),
            elevation_of_terrain: OptionalParameter::omitted(),
            building_address: OptionalParameter::omitted(),
        }
    }

    pub fn elevation_of_ref_height(mut self, elevation_of_ref_height: f64) -> Self {
        self.elevation_of_ref_height = IfcFloat(elevation_of_ref_height).into();
        self
    }

    pub fn elevation_of_terrain(mut self, elevation_of_terrain: f64) -> Self {
        self.elevation_of_terrain = IfcFloat(elevation_of_terrain).into();
        self
    }

    pub fn building_address(
        mut self,
        postal_address: impl Into<IdOr<PostalAddress>>,
        ifc: &mut IFC,
    ) -> Self {
        self.building_address = postal_address.into().or_insert(ifc).id().into();
        self
    }
}

impl RootBuilder for Building {
    fn root_mut(&mut self) -> &mut Root {
        &mut self.spatial_element_structure
    }
}

impl ObjectBuilder for Building {
    fn object_mut(&mut self) -> &mut Object {
        &mut self.spatial_element_structure
    }
}

impl ProductBuilder for Building {
    fn product_mut(&mut self) -> &mut Product {
        &mut self.spatial_element_structure
    }
}

impl SpatialElementBuilder for Building {
    fn spatial_element_mut(&mut self) -> &mut SpatialElement {
        &mut self.spatial_element_structure
    }
}

impl SpatialStructureElementBuilder for Building {
    fn spatial_structure_element_mut(&mut self) -> &mut SpatialStructureElement {
        &mut self.spatial_element_structure
    }
}

impl Deref for Building {
    type Target = SpatialStructureElement;

    fn deref(&self) -> &Self::Target {
        &self.spatial_element_structure
    }
}

impl DerefMut for Building {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.spatial_element_structure
    }
}

impl IFCParse for Building {
    fn parse<'a>() -> impl IFCParser<'a, Self> {
        winnow::seq! {
            Self {
                _: p_space_or_comment_surrounded("IFCBUILDING("),

                spatial_element_structure: SpatialStructureElement::parse(),
                _: Comma::parse(),
                elevation_of_ref_height: OptionalParameter::parse(),
                _: Comma::parse(),
                elevation_of_terrain: OptionalParameter::parse(),
                _: Comma::parse(),
                building_address: OptionalParameter::parse(),

                _: p_space_or_comment_surrounded(");"),
            }
        }
    }
}

impl Display for Building {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "IFCBUILDING({},{},{},{});",
            self.spatial_element_structure,
            self.elevation_of_ref_height,
            self.elevation_of_terrain,
            self.building_address,
        )
    }
}

impl IfcType for Building {}
impl Structure for Building {}

#[cfg(test)]
mod test {
    use winnow::Parser;

    use super::Building;
    use crate::parser::IFCParse;

    #[test]
    fn building_round_trip() {
        let example = "IFCBUILDING('39t4Pu3nTC4ekXYRIHJB9W',#2,'IfcBuilding',$,$,$,$,$,$,$,$,$);";

        let building: Building = Building::parse().parse(example).unwrap();
        let str_building = building.to_string();

        assert_eq!(example, str_building);
    }
}
