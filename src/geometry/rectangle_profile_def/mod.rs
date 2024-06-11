mod deserialize;
mod serialize;

use crate::id::IdOr;
use crate::ifc_type::IfcType;
use crate::parser::ifc_float::IfcFloat;
use crate::parser::label::Label;
use crate::prelude::AxisPlacement;
use crate::IFC;
use crate::{id::Id, parser::optional::OptionalParameter};

use super::profile_type::ProfileType;
/// IfcRectangleProfileDef defines a rectangle as the profile definition used by the swept surface
/// geometry or the swept area solid. It is given by its X extent and its Y extent, and placed within
/// the 2D position coordinate system, established by the Position attribute. It is placed centric
/// within the position coordinate system.
///
/// https://standards.buildingsmart.org/IFC/DEV/IFC4_2/FINAL/HTML/schema/ifcprofileresource/lexical/ifcrectangleprofiledef.htm
pub struct RectangleProfileDef {
    profile_type: ProfileType,
    profile_name: OptionalParameter<Label>,
    position: OptionalParameter<Id>,
    x_dim: IfcFloat,
    y_dim: IfcFloat,
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

    pub fn profile_name(mut self, name: impl Into<Label>) -> Self {
        self.profile_name = name.into().into();
        self
    }

    pub fn position<A: AxisPlacement>(
        mut self,
        position: impl Into<IdOr<A>>,
        ifc: &mut IFC,
    ) -> Self {
        self.position = position.into().into_id(ifc).id().into();
        self
    }
}

impl IfcType for RectangleProfileDef {}

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
