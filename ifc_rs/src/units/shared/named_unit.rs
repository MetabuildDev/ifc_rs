use std::fmt::Display;

use comma::Comma;
use ifc_rs_verify_derive::IfcVerify;
use optional::OptionalParameter;

use crate::{id::Id, parser::*, prelude::*, units::unit_enum::IfcUnitEnum};

/// https://standards.buildingsmart.org/IFC/DEV/IFC4_2/FINAL/HTML/link/ifcnamedunit.htm
#[derive(IfcVerify)]
pub struct NamedUnit {
    /// The dimensional exponents of the SI base units by which the named unit is defined.
    pub dimensions: OptionalParameter<Id>,

    /// The type of the unit.
    pub unit_type: OptionalParameter<IfcUnitEnum>,
}

impl IFCParse for NamedUnit {
    fn parse<'a>() -> impl IFCParser<'a, Self> {
        winnow::seq! {
            Self {
                dimensions: OptionalParameter::parse(),
                _: Comma::parse(),
                unit_type: OptionalParameter::parse(),
            }
        }
    }
}

impl Display for NamedUnit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{},{}", self.dimensions, self.unit_type,)
    }
}
