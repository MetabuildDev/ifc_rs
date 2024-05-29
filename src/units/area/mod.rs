mod deserialize;
mod serialize;

use super::{name::IfcUnitName, prefix::IfcPrefix};

pub struct AreaUnit {
    pub prefix: Option<IfcPrefix>,
    pub name: IfcUnitName,
}

#[cfg(test)]
mod test {
    use winnow::Parser;

    use crate::units::area::AreaUnit;

    #[test]
    fn area_unit_round_trip() {
        let example = "IFCSIUNIT($,.AREAUNIT.,$,.SQUARE_METRE.);";

        let area_unit: AreaUnit = AreaUnit::parse().parse(example).unwrap();
        let str_area_unit = area_unit.to_string();

        assert_eq!(example, str_area_unit);
    }
}
