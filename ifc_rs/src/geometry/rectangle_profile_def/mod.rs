mod deserialize;
mod serialize;

use ifc_rs_verify_derive::IfcVerify;

use crate::id::IdOr;
use crate::parser::real::RealPrimitive;
use crate::parser::string::StringPrimitive;
use crate::prelude::*;
use crate::{id::Id, parser::optional::OptionalParameter};

use super::axis::AxisMappings;
use super::profile_type::ProfileType;

pub struct MappedRectangleProfileDef<'a> {
    pub axis: Option<AxisMappings<'a>>,
    pub x_dim: f64,
    pub y_dim: f64,
}

/// IfcRectangleProfileDef defines a rectangle as the profile definition used by the swept surface
/// geometry or the swept area solid. It is given by its X extent and its Y extent, and placed within
/// the 2D position coordinate system, established by the Position attribute. It is placed centric
/// within the position coordinate system.
///
/// https://standards.buildingsmart.org/IFC/DEV/IFC4_2/FINAL/HTML/schema/ifcprofileresource/lexical/ifcrectangleprofiledef.htm
#[derive(IfcVerify)]
pub struct RectangleProfileDef {
    /// Defines the type of geometry into which this profile definition shall be resolved, either a
    /// curve or a surface area. In case of curve the profile should be referenced by a swept
    /// surface, in case of area the profile should be referenced by a swept area solid.
    pub profile_type: ProfileType,
    /// Human-readable name of the profile, for example according to a standard profile table. As
    /// noted above, machine-readable standardized profile designations should be provided in
    /// IfcExternalReference.ItemReference.
    pub profile_name: OptionalParameter<StringPrimitive>,
    /// Position coordinate system of the parameterized profile definition. If unspecified, no
    /// translation and no rotation is applied.
    #[ifc_types(Axis2D, Axis3D)]
    pub position: OptionalParameter<Id>,
    /// The extent of the rectangle in the direction of the x-axis.
    pub x_dim: RealPrimitive,
    /// The extent of the rectangle in the direction of the y-axis.
    pub y_dim: RealPrimitive,
}

impl RectangleProfileDef {
    pub fn new(profile_type: ProfileType, x_dim: f64, y_dim: f64) -> Self {
        Self {
            profile_type,
            profile_name: OptionalParameter::omitted(),
            position: OptionalParameter::omitted(),
            x_dim: x_dim.into(),
            y_dim: y_dim.into(),
        }
    }

    pub fn profile_name(mut self, name: impl Into<StringPrimitive>) -> Self {
        self.profile_name = name.into().into();
        self
    }

    pub fn position<A: AxisPlacement>(
        mut self,
        position: impl Into<IdOr<A>>,
        ifc: &mut IFC,
    ) -> Self {
        self.position = position.into().or_insert(ifc).id().into();
        self
    }
}

impl<'a> IfcMappedType<'a> for RectangleProfileDef {
    type Target = MappedRectangleProfileDef<'a>;

    fn mappings(&'a self, ifc: &'a IFC) -> Self::Target {
        let axis = self.position.custom().map(|id| {
            let untyped = ifc.data.get_untyped(*id);

            if let Some(d2) = untyped.downcast_ref::<Axis2D>() {
                AxisMappings::map_2d(d2, ifc)
            } else if let Some(d3) = untyped.downcast_ref::<Axis3D>() {
                AxisMappings::map_3d(d3, ifc)
            } else {
                unreachable!("already checked by type checker");
            }
        });

        MappedRectangleProfileDef {
            axis,
            x_dim: self.x_dim.0,
            y_dim: self.y_dim.0,
        }
    }
}

impl IfcType for RectangleProfileDef {}
impl ProfileDef for RectangleProfileDef {}

#[cfg(test)]
mod test {
    use winnow::Parser;

    use crate::parser::IFCParse;

    use super::RectangleProfileDef;

    #[test]
    fn rectangle_profile_def_round_trip() {
        let example = "IFCRECTANGLEPROFILEDEF(.AREA.,$,#250,7.99999999999995E0,3.95);";

        let rectangle_profile_def: RectangleProfileDef =
            RectangleProfileDef::parse().parse(example).unwrap();
        let str_rectangle_profile_def = format!("{rectangle_profile_def}");

        assert_eq!(example, str_rectangle_profile_def);
    }
}
