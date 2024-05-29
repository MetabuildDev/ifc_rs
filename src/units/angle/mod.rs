mod deserialize;
mod serialize;

use super::ConversionUnit;
use crate::{id::Id, parser::optional::OptionalParameter};

pub struct AngleUnit {
    pub dimensional_exponents_id: Id,
    pub parameter_1: OptionalParameter<ConversionUnit>,
    pub parameter_2: OptionalParameter<ConversionUnit>,
}

#[cfg(test)]
mod test {
    use winnow::Parser;

    use crate::units::angle::AngleUnit;

    #[test]
    fn angle_unit_round_trip() {
        let example = "IFCCONVERSIONBASEDUNIT(#52,.PLANEANGLEUNIT.,'DEGREE',#53);";

        let angle_unit: AngleUnit = AngleUnit::parse().parse(example).unwrap();
        let str_angle_unit = format!("{angle_unit}");

        assert_eq!(example, str_angle_unit);
    }
}
