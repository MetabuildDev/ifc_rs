mod deserialize;
mod serialize;

use super::ConversionUnit;
use crate::{
    id::{Id, IdOr},
    parser::optional::OptionalParameter,
};

/// An IfcConversionBasedUnit is used to define a unit that has a conversion rate to a base unit.
/// To identify some commonly used conversion based units, the standard designations
/// (case insensitive) for the Name attribute are indicated in Table 697.
///
/// https://standards.buildingsmart.org/IFC/DEV/IFC4_2/FINAL/HTML/link/ifcconversionbasedunit.htm
pub struct AngleUnit {
    pub dimensional_exponents_id: Id,
    pub parameter_1: OptionalParameter<IdOr<ConversionUnit>>,
    pub parameter_2: OptionalParameter<IdOr<ConversionUnit>>,
}

#[cfg(test)]
mod test {
    use winnow::Parser;

    use crate::parser::optional::IFCParse;
    use crate::units::angle::AngleUnit;

    #[test]
    fn angle_unit_round_trip() {
        let example = "IFCCONVERSIONBASEDUNIT(#52,.PLANEANGLEUNIT.,'DEGREE',#53);";

        let angle_unit: AngleUnit = AngleUnit::parse().parse(example).unwrap();
        let str_angle_unit = format!("{angle_unit}");

        assert_eq!(example, str_angle_unit);
    }
}
