use std::fmt::Display;

use crate::{
    id::Id,
    ifc_type::IfcType,
    parser::{list::IfcList, p_space_or_comment_surrounded, IFCParse, IFCParser},
};

/// IfcUnitAssignment indicates a set of units which may be assigned. Within an
/// IfcUnitAssigment each unit definition shall be unique; that is, there shall
/// be no redundant unit definitions for the same unit type such as length unit
/// or area unit. For currencies, there shall be only a single IfcMonetaryUnit
/// within an IfcUnitAssignment.
///
/// https://standards.buildingsmart.org/IFC/DEV/IFC4_2/FINAL/HTML/link/ifcunitassignment.htm
pub struct UnitAssigment {
    /// Units to be included within a unit assignment.
    pub units: IfcList<Id>,
}

impl IFCParse for UnitAssigment {
    fn parse<'a>() -> impl IFCParser<'a, Self> {
        winnow::seq! {
            Self {
                _: p_space_or_comment_surrounded("IFCUNITASSIGNMENT("),

                units: IfcList::parse(),

                _: p_space_or_comment_surrounded(");"),
            }
        }
    }
}

impl Display for UnitAssigment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "IFCUNITASSIGNMENT({});", self.units)
    }
}

impl IfcType for UnitAssigment {}

#[cfg(test)]
mod test {
    use winnow::Parser;

    use super::UnitAssigment;
    use crate::parser::IFCParse;

    #[test]
    fn rel_aggregates_round_trip() {
        let example = "IFCUNITASSIGNMENT((#18,#19,#20));";

        let unit_assignment: UnitAssigment = UnitAssigment::parse().parse(example).unwrap();
        let str_unit_assignment = unit_assignment.to_string();

        assert_eq!(example, str_unit_assignment);
    }
}
