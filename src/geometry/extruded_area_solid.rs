use crate::geometry::rectangle_profile_def::ProfileDef;
use crate::id::IdOr;
use crate::ifc_type::IfcType;
use crate::prelude::{Axis3D, Direction3D};
use crate::{id::Id, parser::ifc_float::IfcFloat};
use crate::{parser::*, IFC};
use comma::Comma;
use optional::OptionalParameter;

use std::fmt::Display;

use super::shape_representation::ShapeItem;

/// The IfcExtrudedAreaSolid is defined by sweeping a cross section
/// provided by a profile definition. The direction of the extrusion
/// is given by the ExtrudedDirection attribute and the length of the
///  extrusion is given by the Depth attribute. If the planar area has
///  inner boundaries (holes defined), then those holes shall be swept
/// into holes of the solid.
///
/// https://standards.buildingsmart.org/IFC/DEV/IFC4_2/FINAL/HTML/link/ifcextrudedareasolid.htm
pub struct ExtrudedAreaSolid {
    /// The surface defining the area to be swept. It is given as a
    /// profile definition within the xy plane of the position coordinate system.
    pub swept_area: Id,

    /// Position coordinate system for the resulting swept solid of the sweeping
    /// operation. The position coordinate system allows for re-positioning of
    /// the swept solid. If not provided, the swept solid remains within the
    /// position as determined by the cross section or by the directrix used
    /// for the sweeping operation.
    pub position: OptionalParameter<Id>,

    /// The direction in which the surface, provided by SweptArea is to be swept.
    pub extruded_direction: Id,

    /// The distance the surface is to be swept along the ExtrudedDirection.
    pub depth: IfcFloat,
}

impl ExtrudedAreaSolid {
    pub fn new<P: ProfileDef>(
        swept_area: impl Into<IdOr<P>>,
        extruded_direction: impl Into<IdOr<Direction3D>>,
        depth: f64,
        ifc: &mut IFC,
    ) -> Self {
        Self {
            swept_area: swept_area.into().into_id(ifc).id(),
            position: OptionalParameter::omitted(),
            extruded_direction: extruded_direction.into().into_id(ifc).id(),
            depth: depth.into(),
        }
    }

    pub fn position(mut self, position: impl Into<IdOr<Axis3D>>, ifc: &mut IFC) -> Self {
        self.position = position.into().into_id(ifc).id().into();
        self
    }
}

impl IFCParse for ExtrudedAreaSolid {
    fn parse<'a>() -> impl IFCParser<'a, Self> {
        winnow::seq! {
            Self {
                _: p_space_or_comment_surrounded("IFCEXTRUDEDAREASOLID("),

                swept_area: Id::parse(),
                _: Comma::parse(),
                position: OptionalParameter::parse(),
                _: Comma::parse(),
                extruded_direction: Id::parse(),
                _: Comma::parse(),
                depth: IfcFloat::parse(),

                _: p_space_or_comment_surrounded(");"),
            }
        }
    }
}

impl Display for ExtrudedAreaSolid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "IFCEXTRUDEDAREASOLID({},{},{},{});",
            self.swept_area, self.position, self.extruded_direction, self.depth
        )
    }
}

impl IfcType for ExtrudedAreaSolid {}
impl ShapeItem for ExtrudedAreaSolid {}

#[cfg(test)]
mod test {
    use winnow::Parser;

    use crate::parser::IFCParse;

    use super::ExtrudedAreaSolid;

    #[test]
    fn extruded_area_solid_round_trip() {
        let example = "IFCEXTRUDEDAREASOLID(#1457,#1460,#21,2.4384);";

        let area_unit: ExtrudedAreaSolid = ExtrudedAreaSolid::parse().parse(example).unwrap();
        let str_area_unit = area_unit.to_string();

        assert_eq!(example, str_area_unit);
    }
}
