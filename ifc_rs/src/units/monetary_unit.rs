use std::fmt::Display;

use ifc_rs_verify_derive::IfcVerify;

use super::{string::StringPrimitive, IFCParse, IFCParser};
use crate::{prelude::*, units::p_space_or_comment_surrounded};

/// IfcMonetaryUnit is a unit to define currency for money.
///
/// https://standards.buildingsmart.org/IFC/DEV/IFC4_2/FINAL/HTML/link/ifcmonetaryunit.htm
#[derive(IfcVerify)]
pub struct MonetaryUnit {
    /// Code or name of the currency. Permissible values are the three-letter
    /// alphabetic currency codes as per ISO 4217, for example CNY, EUR, GBP, JPY, USD.
    pub currency: StringPrimitive,
}

impl IfcType for MonetaryUnit {}

impl IFCParse for MonetaryUnit {
    fn parse<'a>() -> impl IFCParser<'a, Self> {
        winnow::seq! {
            Self {
                _: p_space_or_comment_surrounded("IFCMONETARYUNIT("),

                currency: StringPrimitive::parse(),

                _: p_space_or_comment_surrounded(");"),
            }
        }
    }
}

impl Display for MonetaryUnit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "IFCMONETARYUNIT({});", self.currency)
    }
}

#[cfg(test)]
mod test {
    use winnow::Parser;

    use super::MonetaryUnit;
    use crate::units::IFCParse;

    #[test]
    fn monetary_unit_round_trip() {
        let example = "IFCMONETARYUNIT('EUR');";

        let parsed: MonetaryUnit = MonetaryUnit::parse().parse(example).unwrap();
        let str = parsed.to_string();

        assert_eq!(example, str);
    }
}
