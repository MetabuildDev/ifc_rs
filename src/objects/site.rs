use std::{
    fmt::Display,
    ops::{Deref, DerefMut},
};

use ifc_verify_derive::IfcVerify;

use crate::{
    id::{IdOr, TypedId},
    ifc_type::{IfcType, IfcVerify},
    parser::{
        comma::Comma, ifc_float::IfcFloat, ifc_integer::IfcInteger, label::Label, list::IfcList,
        optional::OptionalParameter, p_space_or_comment_surrounded, IFCParse, IFCParser,
    },
    prelude::*,
};

/// A site is a defined area of land, possibly covered with water, on which the
/// project construction is to be completed. A site may be used to erect,
/// retrofit or turn down building(s), or for other construction related
/// developments.
///
/// A site may include a definition of the single geographic reference point
/// for this site (global position using WGS84 with Longitude, Latitude and
/// Elevation). The precision is provided up to millionth of a second and it
/// provides an absolute placement in relation to the real world as used in
/// exchange with geospational information systems. If asserted, the Longitude,
/// Latitude and Elevation establish the point in WGS84 where the point 0.,0.,0.
/// of the LocalPlacement of IfcSite is situated.
///
/// The geometrical placement of the site, defined by the IfcLocalPlacement,
/// shall be always relative to the spatial structure element, in which this
/// site is included, or absolute, i.e. to the world coordinate system, as
/// established by the geometric representation context of the project. The
/// world coordinate system, established at the IfcProject.RepresentationContexts,
/// may include a definition of the true north within the XY plane of the
/// world coordinate system, if provided, it can be obtained at
/// IfcGeometricRepresentationContext.TrueNorth.
///
/// A project may span over several connected or disconnected sites. Therefore
/// site complex provides for a collection of sites included in a project. A
/// site can also be decomposed in parts, where each part defines a site section.
/// This is defined by the composition type attribute of the supertype
/// IfcSpatialStructureElements which is interpreted as follow:
///
/// * COMPLEX = site complex
/// * ELEMENT = site
/// * PARTIAL = site section
///
/// The IfcSite is used to build the spatial structure of a building (that
/// serves as the primary project breakdown and is required to be hierarchical).
///
/// https://standards.buildingsmart.org/IFC/DEV/IFC4_2/FINAL/HTML/schema/ifcproductextension/lexical/ifcsite.htm
#[derive(IfcVerify)]
pub struct Site {
    spatial_element_structure: SpatialStructureElement,

    /// World Latitude at reference point (most likely defined in legal
    /// description). Defined as integer values for degrees, minutes, seconds,
    /// and, optionally, millionths of seconds with respect to the world
    /// geodetic system WGS84.
    pub ref_latitude: OptionalParameter<IfcList<IfcInteger>>, //TODO: CompoundPlaneAngleMeasure

    /// World Longitude at reference point (most likely defined in legal
    /// description). Defined as integer values for degrees, minutes, seconds,
    /// and, optionally, millionths of seconds with respect to the world
    /// geodetic system WGS84.
    pub ref_longitude: OptionalParameter<IfcList<IfcInteger>>, //TODO: CompoundPlaneAngleMeasure

    /// Datum elevation relative to sea level.
    pub ref_elevation: OptionalParameter<IfcFloat>,

    /// The land title number (designation of the site within a regional system).
    pub land_title_number: OptionalParameter<Label>,

    /// Address given to the site for postal purposes.
    pub site_address: OptionalParameter<TypedId<PostalAddress>>,
}

impl Site {
    pub fn new<'a>(name: impl Into<Label>) -> Self {
        Self {
            spatial_element_structure: SpatialStructureElement::new(SpatialElement::new(
                Product::new(Object::new(Root::new(name.into()))),
            )),
            ref_latitude: OptionalParameter::omitted(),
            ref_longitude: OptionalParameter::omitted(),
            ref_elevation: OptionalParameter::omitted(),
            land_title_number: OptionalParameter::omitted(),
            site_address: OptionalParameter::omitted(),
        }
    }

    pub fn ref_latitude(mut self, ref_latitude: Vec<i64>) -> Self {
        self.ref_latitude = IfcList(ref_latitude.iter().map(|&i| i.into()).collect()).into();
        self
    }

    pub fn ref_longitude(mut self, ref_longitude: Vec<i64>) -> Self {
        self.ref_longitude = IfcList(ref_longitude.iter().map(|&i| i.into()).collect()).into();
        self
    }

    pub fn ref_elevation(mut self, ref_elevation: f64) -> Self {
        self.ref_elevation = IfcFloat(ref_elevation).into();
        self
    }

    pub fn land_title_number(mut self, land_title_number: impl Into<Label>) -> Self {
        self.land_title_number = land_title_number.into().into();
        self
    }

    pub fn site_address(
        mut self,
        postal_address: impl Into<IdOr<PostalAddress>>,
        ifc: &mut IFC,
    ) -> Self {
        self.site_address = postal_address.into().or_insert(ifc).into();
        self
    }
}

impl RootBuilder for Site {
    fn root_mut(&mut self) -> &mut Root {
        &mut self.spatial_element_structure
    }
}

impl ObjectBuilder for Site {
    fn object_mut(&mut self) -> &mut Object {
        &mut self.spatial_element_structure
    }
}

impl ProductBuilder for Site {
    fn product_mut(&mut self) -> &mut Product {
        &mut self.spatial_element_structure
    }
}

impl SpatialElementBuilder for Site {
    fn spatial_element_mut(&mut self) -> &mut SpatialElement {
        &mut self.spatial_element_structure
    }
}

impl SpatialStructureElementBuilder for Site {
    fn spatial_structure_element_mut(&mut self) -> &mut SpatialStructureElement {
        &mut self.spatial_element_structure
    }
}

impl Deref for Site {
    type Target = SpatialStructureElement;

    fn deref(&self) -> &Self::Target {
        &self.spatial_element_structure
    }
}

impl DerefMut for Site {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.spatial_element_structure
    }
}

impl IFCParse for Site {
    fn parse<'a>() -> impl IFCParser<'a, Self> {
        winnow::seq! {
            Self {
                _: p_space_or_comment_surrounded("IFCSITE("),

                spatial_element_structure: SpatialStructureElement::parse(),
                _: Comma::parse(),
                ref_latitude: OptionalParameter::parse(),
                _: Comma::parse(),
                ref_longitude: OptionalParameter::parse(),
                _: Comma::parse(),
                ref_elevation: OptionalParameter::parse(),
                _: Comma::parse(),
                land_title_number: OptionalParameter::parse(),
                _: Comma::parse(),
                site_address: OptionalParameter::parse(),

                _: p_space_or_comment_surrounded(");"),
            }
        }
    }
}

impl Display for Site {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "IFCSITE({},{},{},{},{},{});",
            self.spatial_element_structure,
            self.ref_latitude,
            self.ref_longitude,
            self.ref_elevation,
            self.land_title_number,
            self.site_address
        )
    }
}

impl IfcType for Site {}
impl Structure for Site {}

#[cfg(test)]
mod test {
    use winnow::Parser;

    use super::Site;
    use crate::parser::IFCParse;

    #[test]
    fn site_round_trip() {
        let example = "IFCSITE('1b0aQcLSb6KP4cEKR_kZZp',#12,'Gelaende 0815','No real site','LandUse',#104,#1289,$,.ELEMENT.,(49,9,0,0),(8,43,0,0),0.,$,$);";

        let site = Site::parse().parse(example).unwrap();
        let str_site = site.to_string();

        assert_eq!(example, str_site);
    }
}
