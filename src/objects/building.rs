use std::{fmt::Display, ops::Deref};

use super::{
    address::PostalAddress,
    owner_history::OwnerHistory,
    shared::{
        composition_type_enum::CompositionTypeEnum, object::Object, product::Product, root::Root,
        spatial_element::SpatialElement, spatial_structure_element::SpatialStructureElement,
    },
};
use crate::{
    geometry::{local_placement::LocalPlacement, product_definition_shape::ProductDefinitionShape},
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
    pub fn new<'a>(
        global_id: impl Into<Label>,
        owner_history: impl Into<Option<IdOr<OwnerHistory>>>,
        name: impl Into<Option<&'a str>>,
        description: impl Into<Option<&'a str>>,
        object_type: impl Into<Option<&'a str>>,
        object_placement: impl Into<Option<IdOr<LocalPlacement>>>,
        representation: impl Into<Option<IdOr<ProductDefinitionShape>>>,
        composition_type_enum: impl Into<Option<CompositionTypeEnum>>,
        elevation_of_ref_height: impl Into<Option<f64>>,
        elevation_of_terrain: impl Into<Option<f64>>,
        building_address: impl Into<Option<IdOr<PostalAddress>>>,
        ifc: &mut IFC,
    ) -> Self {
        Self {
            spatial_element_structure: SpatialStructureElement::new(
                SpatialElement::new(
                    Product::new(
                        Object::new(
                            Root::new(
                                global_id.into(),
                                owner_history.into().map(|h| h.into_id(ifc).id()).into(),
                                name.into().map(|s| s.into()).into(),
                                description.into().map(|s| s.into()).into(),
                            ),
                            object_type.into().map(|s| s.into()).into(),
                        ),
                        object_placement
                            .into()
                            .map(|p| IdOr::Id(p.into_id(ifc).id()))
                            .into(),
                        representation.into().map(|r| r.into_id(ifc).id()).into(),
                    ),
                    OptionalParameter::omitted(),
                ),
                composition_type_enum.into().into(),
            ),
            elevation_of_ref_height: elevation_of_ref_height.into().map(|f| f.into()).into(),
            elevation_of_terrain: elevation_of_terrain.into().map(|f| f.into()).into(),
            building_address: building_address.into().map(|a| a.into_id(ifc).id()).into(),
        }
    }
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
