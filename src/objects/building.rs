use std::{fmt::Display, ops::Deref};

use super::{address::PostalAddress, shared::spatial_structure_element::SpatialStructureElement};
use crate::parser::{
    ifc_float::IfcFloat, optional::OptionalParameter, p_space_or_comment_surrounded, IFCParse,
    IFCParser,
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
    pub building_address: OptionalParameter<PostalAddress>,
}

impl Deref for Building {
    type Target = SpatialStructureElement;

    fn deref(&self) -> &Self::Target {
        &self.spatial_element_structure
    }
}

impl IFCParse for Building {
    fn parse<'a>() -> impl IFCParser<'a, Self> {
        winnow::seq! {
            Self {
                _: p_space_or_comment_surrounded("IFCBUILDING("),

                spatial_element_structure: SpatialStructureElement::parse(),
                _: p_space_or_comment_surrounded(","),
                elevation_of_ref_height: OptionalParameter::parse(),
                _: p_space_or_comment_surrounded(","),
                elevation_of_terrain: OptionalParameter::parse(),
                _: p_space_or_comment_surrounded(","),
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
