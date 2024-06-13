mod deserialize;
mod serialize;

use crate::geometry::indexed_poly_curve::Curve;
use crate::geometry::rectangle_profile_def::ProfileDef;
use crate::id::IdOr;
use crate::ifc_type::IfcType;
use crate::parser::label::Label;
use crate::IFC;
use crate::{id::Id, parser::optional::OptionalParameter};

use super::profile_type::ProfileType;

/// The closed profile IfcArbitraryClosedProfileDef defines an arbitrary
/// two-dimensional profile for the use within the swept surface geometry,
/// the swept area solid or a sectioned spine. It is given by an outer
/// boundary from which the surface or solid can be constructed.
///
/// https://standards.buildingsmart.org/IFC/DEV/IFC4_2/FINAL/HTML/schema/ifcprofileresource/lexical/ifcarbitraryclosedprofiledef.htm
pub struct ArbitraryClosedProfileDef {
    /// Defines the type of geometry into which this profile definition shall be resolved, either a
    /// curve or a surface area. In case of curve the profile should be referenced by a swept
    /// surface, in case of area the profile should be referenced by a swept area solid.
    profile_type: ProfileType,
    /// Human-readable name of the profile, for example according to a standard profile table. As
    /// noted above, machine-readable standardized profile designations should be provided in
    /// IfcExternalReference.ItemReference.
    profile_name: OptionalParameter<Label>,
    /// `IfcCurve` Bounded curve, defining the outer boundaries of the arbitrary profile.
    // TODO: this should be `TypeId<C: Curve>` see issue #57
    outer_curve: Id,
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
            outer_curve: outer_curve.into().into_id(ifc).id(),
        }
    }

    pub fn profile_name(mut self, name: impl Into<Label>) -> Self {
        self.profile_name = name.into().into();
        self
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

        let profile_def: ArbitraryClosedProfileDef =
            ArbitraryClosedProfileDef::parse().parse(example).unwrap();
        let str_profile_def = format!("{profile_def}");

        assert_eq!(example, str_profile_def);
    }
}
