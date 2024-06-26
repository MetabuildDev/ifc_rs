use ifc_rs_verify_derive::IfcVerify;

use crate::{
    id::Id,
    parser::{bool::IfcBool, label::Label, optional::OptionalParameter},
    prelude::*,
};

mod deserialize;
mod serialize;

/// The IfcIndexedPolyCurve is a bounded curve with only linear and circular
/// arc segments defined by a Cartesian point list and an optional list of
/// segments, providing indices into the Cartesian point list. In the case
/// that the list of Segments is not provided, all points in the
/// IfcCartesianPointList are connected by straight line segments in the
/// order they appear in the IfcCartesianPointList.
#[derive(IfcVerify)]
pub struct IndexedPolyCurve {
    /// A list of points, provided by a point list of either two, or three
    /// dimensions, that is used to define the poly curve. If the attribute
    /// Segments is not provided, the poly curve is generated as a poly line
    /// by connecting the points in the order of their appearance in the point
    /// list. If the attribute Segments is provided, the segments determine,
    /// how the points are to be used to create straigth and circular
    /// arc segments.
    #[ifc_types(PointList2D, PointList3D)]
    pub points: Id,
    /// List of straight line and circular arc segments, each providing a
    /// list of indices into the Cartesian point list. Indices should
    /// preserve consecutive connectivity between the segments, the start
    /// index of the next segment shall be identical with the end index
    /// of the previous segment.
    pub segments: OptionalParameter<Label>, // TODO: this is either `LineIndex` or `ArcIndex`
    /// Indication of whether the curve intersects itself or not; this is for
    /// information only.
    pub self_intersect: OptionalParameter<IfcBool>,
}

impl IndexedPolyCurve {
    pub fn new(point_list: impl PointList, ifc: &mut IFC) -> Self {
        let id = ifc.data.insert_new(point_list);

        Self {
            points: id.id(),
            segments: OptionalParameter::omitted(),
            self_intersect: OptionalParameter::omitted(),
        }
    }
}

impl IfcType for IndexedPolyCurve {}
impl Curve for IndexedPolyCurve {}
