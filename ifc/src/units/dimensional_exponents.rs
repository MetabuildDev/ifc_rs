use std::fmt::Display;

use ifc_verify_derive::IfcVerify;
use winnow::ascii::dec_int;

use crate::{
    ifc_type::{IfcType, IfcVerify},
    units::{comma::Comma, p_space_or_comment_surrounded},
    IFC,
};

use super::IFCParse;

// ?
type ExponentType = i32;

/// The dimensionality of any quantity can be expressed as a product of powers of the dimensions of
/// base quantities. The dimensional exponents entity defines the powers of the dimensions of the
/// base quantities. All the physical quantities are founded on seven base quantities (ISO 31
/// (clause 2)).
///
/// # NOTE
/// Length, mass, time, electric current, thermodynamic temperature, amount of substance, and
/// luminous intensity are the seven base quantities.
///
/// # EXAMPLE
/// A length of 2 millimetres has a length exponent of 1. The remaining exponents are equal to 0.
///
/// # EXAMPLE
/// A velocity of 2 millimetres per second has a length exponent of 1 and a time exponent of -1.
/// The remaining exponents are equal to 0.
#[derive(IfcVerify)]
pub struct DimensionalExponents {
    /// The power of the length base quantity.
    pub length: ExponentType,
    /// The power of the mass base quantity.
    pub mass: ExponentType,
    /// The power of the time base quantity.
    pub time: ExponentType,
    /// The power of the electric current base quantity.
    pub electric_current: ExponentType,
    /// The power of the thermodynamic temperature base quantity.
    pub thermodynamic_temperature: ExponentType,
    /// The power of the amount of substance base quantity.
    pub amount_of_substance: ExponentType,
    /// The power of the luminous intensity base quantity.
    pub luminouse_intensity: ExponentType,
}

impl Display for DimensionalExponents {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "IFCDIMENSIONALEXPONENTS({},{},{},{},{},{},{});",
            self.length,
            self.mass,
            self.time,
            self.electric_current,
            self.thermodynamic_temperature,
            self.amount_of_substance,
            self.luminouse_intensity
        )
    }
}

impl IfcType for DimensionalExponents {}

impl IFCParse for DimensionalExponents {
    fn parse<'a>() -> impl super::IFCParser<'a, Self>
    where
        Self: Sized,
    {
        winnow::seq! {
            Self {
                _: p_space_or_comment_surrounded("IFCDIMENSIONALEXPONENTS("),
                length: dec_int,
                _: Comma::parse(),
                mass: dec_int,
                _: Comma::parse(),
                time: dec_int,
                _: Comma::parse(),
                electric_current: dec_int,
                _: Comma::parse(),
                thermodynamic_temperature: dec_int,
                _: Comma::parse(),
                amount_of_substance: dec_int,
                _: Comma::parse(),
                luminouse_intensity: dec_int,
                _: p_space_or_comment_surrounded(");"),
            }
        }
    }
}

#[cfg(test)]
mod test {
    use winnow::Parser;

    use super::DimensionalExponents;
    use crate::units::IFCParse;

    #[test]
    fn dimensional_exponent_round_trip() {
        let example = "IFCDIMENSIONALEXPONENTS(0,0,0,0,0,0,0);";

        let parsed = DimensionalExponents::parse().parse(example).unwrap();
        let str = parsed.to_string();

        assert_eq!(example, str);
    }
}
