use crate::id::Id;

mod deserialize;
mod serialize;

pub struct ExtrudedAreaSolid {
    pub swept_area: Id,
    pub position: Id,
    pub extruded_direction: Id,
    pub depth: f64,
}

#[cfg(test)]
mod test {
    use winnow::Parser;

    use super::ExtrudedAreaSolid;
    use crate::parser::optional::IFCParse;

    #[test]
    fn extruded_area_solid_round_trip() {
        let example = "IFCEXTRUDEDAREASOLID(#1457,#1460,#21,2.4384);";

        let area_unit: ExtrudedAreaSolid = ExtrudedAreaSolid::parse().parse(example).unwrap();
        let str_area_unit = area_unit.to_string();

        assert_eq!(example, str_area_unit);
    }
}
