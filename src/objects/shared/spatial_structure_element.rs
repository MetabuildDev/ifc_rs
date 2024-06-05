use std::{fmt::Display, ops::Deref};

use crate::parser::{comma::Comma, optional::OptionalParameter, IFCParse, IFCParser};

use super::{composition_type_enum::CompositionTypeEnum, spatial_element::SpatialElement};

/// A spatial element is the generalization of all spatial elements that
/// might be used to define a spatial structure or to define spatial zones.
///
/// https://standards.buildingsmart.org/IFC/DEV/IFC4_2/FINAL/HTML/schema/ifcproductextension/lexical/ifcspatialelement.htm
pub struct SpatialStructureElement {
    spatial_element: SpatialElement,

    /// Denotes, whether the predefined spatial structure element
    /// represents itself, or an aggregate (complex) or a part (part).
    /// The interpretation is given separately for each subtype of
    /// spatial structure element. If no CompositionType is asserted,
    /// the dafault value 'ELEMENT' applies.
    pub composition_type: OptionalParameter<CompositionTypeEnum>,
}

impl Deref for SpatialStructureElement {
    type Target = SpatialElement;

    fn deref(&self) -> &Self::Target {
        &self.spatial_element
    }
}

impl IFCParse for SpatialStructureElement {
    fn parse<'a>() -> impl IFCParser<'a, Self> {
        winnow::seq! {
            Self {
                spatial_element: SpatialElement::parse(),
                _: Comma::parse(),
                composition_type: OptionalParameter::parse(),
            }
        }
    }
}

impl Display for SpatialStructureElement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{},{}", self.spatial_element, self.composition_type)
    }
}
