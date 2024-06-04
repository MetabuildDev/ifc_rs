use crate::{id::Id, parser::ifc_float::IfcFloat};

mod deserialize;
mod serialize;

pub struct ExtrudedAreaSolid {
    pub swept_area: Id,
    pub position: Id,
    pub extruded_direction: Id,
    pub depth: IfcFloat,
}

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
