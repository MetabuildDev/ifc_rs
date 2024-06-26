pub(crate) mod assignment;
pub(crate) mod conversion_based_unit;
pub(crate) mod derived_unit;
pub(crate) mod derived_unit_element;
pub(crate) mod derived_unit_enum;
pub(crate) mod dimensional_exponents;
pub(crate) mod measure;
pub(crate) mod measure_with_unit;
pub(crate) mod monetary_unit;
pub(crate) mod name;
pub(crate) mod prefix;
pub(crate) mod prelude;
pub(crate) mod shared;
pub(crate) mod si_unit;
pub(crate) mod unit_enum;

use crate::{parser::*, traits::prelude::IfcType};
use winnow::combinator::alt;

/// accumulator parser for all unit types into an opaque `dyn IfcType`
pub struct Units;

impl Units {
    /// parser for any unit type into an opaque `dyn IfcType`
    pub fn parse<'a>() -> impl IFCParser<'a, Box<dyn IfcType>> {
        alt((
            assignment::UnitAssigment::parse_any(),
            conversion_based_unit::ConversionBasedUnit::parse_any(),
            si_unit::SiUnit::parse_any(),
            dimensional_exponents::DimensionalExponents::parse_any(),
            measure_with_unit::MeasureWithUnit::parse_any(),
            measure::Measures::parse(),
            derived_unit::DerivedUnit::parse_any(),
            derived_unit_element::DerivedUnitElement::parse_any(),
            monetary_unit::MonetaryUnit::parse_any(),
        ))
    }
}
