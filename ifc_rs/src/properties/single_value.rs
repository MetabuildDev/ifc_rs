use std::fmt::Display;

use comma::Comma;
use ifc_rs_verify_derive::IfcVerify;
use optional::OptionalParameter;
use string::StringPrimitive;

use crate::{parser::*, prelude::*};

/// The property with a single value IfcPropertySingleValue defines a property object which has a
/// single (numeric or descriptive) value assigned. It defines a property - single value
/// combination for which the property Name, an optional Description, and an optional NominalValue
/// with measure type is provided. In addition, the default unit as specified within the project
/// unit context can be overriden by assigning an Unit.
///
/// The unit is handled by the Unit attribute, see Table 704 for an example of various single value
/// properties:
///
/// - If the Unit attribute is not given, then the unit is already implied by the type of
///   IfcMeasureValue or IfcDerivedMeasureValue. The associated unit can be found at the
///   IfcUnitAssignment globally defined at the project level (IfcProject.UnitsInContext).
/// - If the Unit attribute is given, then the unit assigned by the Unit attribute overrides the
///   globally assigned unit.
///
/// https://standards.buildingsmart.org/IFC/DEV/IFC4_2/FINAL/HTML/schema/ifcpropertyresource/lexical/ifcpropertysinglevalue.htm
#[derive(IfcVerify)]
pub struct PropertySingleValue {
    base: PropertyBase,

    /// Value and measure type of this property.
    pub value: IfcValue,
    /// Unit for the nominal value, if not given, the default value for the measure type (given by
    /// the TYPE of nominal value) is used as defined by the global unit assignment at IfcProject.
    pub unit: OptionalParameter<Id>,
}

impl PropertySingleValue {
    pub fn new(name: impl Into<StringPrimitive>, value: IfcValue, unit: Option<Id>) -> Self {
        Self {
            base: PropertyBase::new(name),
            value,
            unit: unit.map_or_else(OptionalParameter::omitted, |val| val.into()),
        }
    }
}

impl PropertyBuilder for PropertySingleValue {
    fn property_mut(&mut self) -> &mut PropertyBase {
        &mut self.base
    }
}

impl IFCParse for PropertySingleValue {
    fn parse<'a>() -> impl IFCParser<'a, Self>
    where
        Self: Sized,
    {
        winnow::seq! {
            Self {
                _: p_space_or_comment_surrounded("IFCPROPERTYSINGLEVALUE("),

                base: PropertyBase::parse(),
                _: Comma::parse(),
                value: IfcValue::parse(),
                _: Comma::parse(),
                unit: OptionalParameter::parse(),

                _: p_space_or_comment_surrounded(");")
            }
        }
    }
}

impl Display for PropertySingleValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "IFCPROPERTYSINGLEVALUE({},{},{});",
            self.base, self.value, self.unit
        )
    }
}

impl IfcType for PropertySingleValue {}

#[cfg(test)]
mod test {
    use winnow::Parser;

    use super::{IFCParse, PropertySingleValue};

    #[test]
    fn property_single_value_bool_round_trip() {
        let example = "IFCPROPERTYSINGLEVALUE('Combustible','Combustible',IFCBOOLEAN(.FALSE.),$);";

        let single_value = PropertySingleValue::parse().parse(example).unwrap();
        let str_single_value = single_value.to_string();

        assert_eq!(example, str_single_value);
    }

    #[test]
    fn property_single_value_label_round_trip() {
        let example = "IFCPROPERTYSINGLEVALUE('AcousticRating','AcousticRating',IFCLABEL(''),$);";

        let single_value = PropertySingleValue::parse().parse(example).unwrap();
        let str_single_value = single_value.to_string();

        assert_eq!(example, str_single_value);
    }

    #[test]
    fn property_single_value_id_round_trip() {
        let example = "IFCPROPERTYSINGLEVALUE('Reference','Reference',IFCIDENTIFIER(''),$);";

        let single_value = PropertySingleValue::parse().parse(example).unwrap();
        let str_single_value = single_value.to_string();

        assert_eq!(example, str_single_value);
    }
}
