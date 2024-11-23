use std::{
    fmt::Display,
    ops::{Deref, DerefMut},
};

use ifc_rs_verify_derive::IfcVerify;

use crate::{
    parser::{comma::Comma, optional::OptionalParameter, IFCParse, IFCParser},
    prelude::*,
};

use super::{composition_type_enum::CompositionTypeEnum, spatial_element::SpatialElement};

/// A spatial element is the generalization of all spatial elements that
/// might be used to define a spatial structure or to define spatial zones.
///
/// https://standards.buildingsmart.org/IFC/DEV/IFC4_2/FINAL/HTML/schema/ifcproductextension/lexical/ifcspatialelement.htm
#[derive(IfcVerify)]
pub struct SpatialStructureElement {
    #[inherited]
    spatial_element: SpatialElement,

    /// Denotes, whether the predefined spatial structure element
    /// represents itself, or an aggregate (complex) or a part (part).
    /// The interpretation is given separately for each subtype of
    /// spatial structure element. If no CompositionType is asserted,
    /// the dafault value 'ELEMENT' applies.
    pub composition_type: OptionalParameter<CompositionTypeEnum>,
}

impl SpatialStructureElement {
    pub fn new(spatial_element: SpatialElement) -> Self {
        Self {
            spatial_element,
            composition_type: OptionalParameter::omitted(),
        }
    }
}

pub trait SpatialStructureElementBuilder: Sized {
    fn spatial_structure_element_mut(&mut self) -> &mut SpatialStructureElement;

    fn composition_type(mut self, composition_type: CompositionTypeEnum) -> Self {
        self.spatial_structure_element_mut().composition_type = composition_type.into();
        self
    }
}

impl Deref for SpatialStructureElement {
    type Target = SpatialElement;

    fn deref(&self) -> &Self::Target {
        &self.spatial_element
    }
}

impl DerefMut for SpatialStructureElement {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.spatial_element
    }
}

impl IFCParse for SpatialStructureElement {
    fn parse<'a>() -> impl IFCParser<'a, Self> {
        winnow::seq! {
            SpatialStructureElement {
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
