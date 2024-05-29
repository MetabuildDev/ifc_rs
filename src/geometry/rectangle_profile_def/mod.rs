mod deserialize;
mod serialize;

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
    position: OptionalParameter<Id>,
    x_dim: f64,
    y_dim: f64,
}

#[cfg(test)]
mod test {
    use winnow::Parser;

    use super::RectangleProfileDef;
    use crate::parser::optional::IFCParse;

    #[test]
    fn rectangle_profile_def_round_trip() {
        let example = "IFCRECTANGLEPROFILEDEF(.AREA.,$,#250,7.99999999999995,3.95);";

        let rectangle_profile_def: RectangleProfileDef =
            RectangleProfileDef::parse().parse(example).unwrap();
        let str_rectangle_profile_def = format!("{rectangle_profile_def}");

        assert_eq!(example, str_rectangle_profile_def);
    }
}
