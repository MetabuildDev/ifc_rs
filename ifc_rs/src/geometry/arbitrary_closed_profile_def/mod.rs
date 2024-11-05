mod deserialize;
mod serialize;

use bevy_math::{DVec2, DVec3};
use ifc_rs_verify_derive::IfcVerify;

use crate::{
    id::IdOr,
    parser::{optional::OptionalParameter, string::StringPrimitive},
    prelude::*,
};

#[derive(Debug)]
pub enum Points {
    D2(Vec<DVec2>),
    D3(Vec<DVec3>),
}

pub struct MappedArbitraryClosedProfileDef {
    pub profile_type: ProfileType,
    pub points: Points,
}

/// The closed profile IfcArbitraryClosedProfileDef defines an arbitrary
/// two-dimensional profile for the use within the swept surface geometry,
/// the swept area solid or a sectioned spine. It is given by an outer
/// boundary from which the surface or solid can be constructed.
///
/// https://standards.buildingsmart.org/IFC/DEV/IFC4_2/FINAL/HTML/schema/ifcprofileresource/lexical/ifcarbitraryclosedprofiledef.htm
#[derive(IfcVerify)]
pub struct ArbitraryClosedProfileDef {
    /// Defines the type of geometry into which this profile definition shall be resolved, either a
    /// curve or a surface area. In case of curve the profile should be referenced by a swept
    /// surface, in case of area the profile should be referenced by a swept area solid.
    pub profile_type: ProfileType,
    /// Human-readable name of the profile, for example according to a standard profile table. As
    /// noted above, machine-readable standardized profile designations should be provided in
    /// IfcExternalReference.ItemReference.
    pub profile_name: OptionalParameter<StringPrimitive>,
    /// `IfcCurve` Bounded curve, defining the outer boundaries of the arbitrary profile.
    #[ifc_types(IndexedPolyCurve, PolyLine)]
    pub outer_curve: Id,
}

impl ArbitraryClosedProfileDef {
    pub fn new<C: Curve>(
        profile_type: ProfileType,
        outer_curve: impl Into<IdOr<C>>,
        ifc: &mut IFC,
    ) -> Self {
        Self {
            profile_type,
            profile_name: OptionalParameter::omitted(),
            outer_curve: outer_curve.into().or_insert(ifc).id(),
        }
    }

    pub fn profile_name(mut self, name: impl Into<StringPrimitive>) -> Self {
        self.profile_name = name.into().into();
        self
    }
}

impl<'a> IfcMappedType<'a> for ArbitraryClosedProfileDef {
    type Target = MappedArbitraryClosedProfileDef;

    fn mappings(&'a self, ifc: &'a IFC) -> Self::Target {
        MappedArbitraryClosedProfileDef {
            profile_type: self.profile_type,

            points: {
                let untyped = ifc.data.get_untyped(self.outer_curve);

                if let Some(indexed_poly_curve) = untyped.downcast_ref::<IndexedPolyCurve>() {
                    let untyped_points = ifc.data.get_untyped(indexed_poly_curve.points);

                    if let Some(list_2d) = untyped_points.downcast_ref::<PointList2D>() {
                        Points::D2(list_2d.coord_list.0.iter().map(|point| point.0).collect())
                    } else if let Some(list_3d) = untyped_points.downcast_ref::<PointList3D>() {
                        Points::D3(list_3d.coord_list.0.iter().map(|point| point.0).collect())
                    } else {
                        unreachable!("checked by type checker");
                    }
                } else if let Some(poly_line) = untyped.downcast_ref::<PolyLine>() {
                    poly_line.points(ifc)
                } else {
                    unreachable!("checked by type checker");
                }
            },
        }
    }
}

impl IfcType for ArbitraryClosedProfileDef {}
impl ProfileDef for ArbitraryClosedProfileDef {}

#[cfg(test)]
mod test {
    use winnow::Parser;

    use crate::geometry::arbitrary_closed_profile_def::ArbitraryClosedProfileDef;
    use crate::parser::IFCParse;

    #[test]
    fn arbitrary_closed_profile_def_round_trip() {
        let example = "IFCARBITRARYCLOSEDPROFILEDEF(.AREA.,$,#25);";

        let profile_def = ArbitraryClosedProfileDef::parse().parse(example).unwrap();
        let str_profile_def = format!("{profile_def}");

        assert_eq!(example, str_profile_def);
    }
}
