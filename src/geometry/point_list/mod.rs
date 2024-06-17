mod deserialize;
mod serialize;

use ifc_type_derive::IfcVerify;

use crate::{
    ifc_type::{IfcType, IfcVerify},
    parser::{
        ifc_float::{IfcDVec2, IfcDVec3},
        label::Label,
        list::IfcList,
        optional::OptionalParameter,
    },
    IFC,
};

/// The IfcCartesianPointList2D defines an ordered collection of two-dimentional
/// Cartesian points. Each Cartesian point is provided as an two-dimensional
/// point by a fixed list of two coordinates. The attribute CoordList is a
/// two-dimensional list, where
///
/// * first dimension is an unbounded list representing each 2D Cartesian point;
/// * second dimension is a fixed list of two list members, where [1] is the
/// x-coordinate, and [2] the y-coordinate of the Cartesian point.
///
/// https://standards.buildingsmart.org/IFC/DEV/IFC4_2/FINAL/HTML/schema/ifcgeometricmodelresource/lexical/ifccartesianpointlist2d.htm
#[derive(Debug, Clone, IfcVerify)]
pub struct PointList2D {
    pub coord_list: IfcList<IfcDVec2>,
    pub tag_list: OptionalParameter<IfcList<Label>>,
}

impl PointList2D {
    pub fn new(coord_list: impl Iterator<Item = impl Into<IfcDVec2>>) -> Self {
        Self {
            coord_list: IfcList(coord_list.map(|p| p.into()).collect()),
            tag_list: OptionalParameter::omitted(),
        }
    }
}

impl IfcType for PointList2D {}

/// The IfcCartesianPointList3D defines an ordered collection of two-dimentional
/// Cartesian points. Each Cartesian point is provided as an two-dimensional
/// point by a fixed list of three coordinates. The attribute CoordList is a
/// two-dimensional list, where
///
/// * first dimension is an unbounded list representing each 3D Cartesian point;
/// * second dimension is a fixed list of two list members, where [1] is the
/// x-coordinate, [2] is the y-coord, and [3] the z-coordinate of the Cartesian point.
///
/// https://standards.buildingsmart.org/IFC/DEV/IFC4_2/FINAL/HTML/schema/ifcgeometricmodelresource/lexical/ifccartesianpointlist3d.htm
#[derive(Debug, Clone, IfcVerify)]
pub struct PointList3D {
    pub coord_list: IfcList<IfcDVec3>,
    pub tag_list: OptionalParameter<IfcList<Label>>,
}

impl PointList3D {
    pub fn new(coord_list: impl Iterator<Item = impl Into<IfcDVec3>>) -> Self {
        Self {
            coord_list: IfcList(coord_list.map(|p| p.into()).collect()),
            tag_list: OptionalParameter::omitted(),
        }
    }
}

impl IfcType for PointList3D {}

pub trait PointList: IfcType {}
impl PointList for PointList2D {}
impl PointList for PointList3D {}
