use std::str::FromStr;

use optional::OptionalParameter;
use place_holder::Inherited;
use strum::{Display, EnumString, VariantNames};
use winnow::ascii::dec_int;
use winnow::combinator::{alt, delimited};
use winnow::Parser;

use crate::parser::*;

/// The IfcDimensionCount is restricted to have the dimensionality of either 1, 2, or 3
/// - the WR1 had been added as an addition to the STEP P42 entity dimension_count.
/// In contrary to the STEP P42 constraint, that all geometric representation items
/// within a geometric representation context are forced to have the same dimension count,
/// the IFC geometry allows mixed dimensions, particularly when defining the boundary of planar surfaces.
///
/// https://standards.buildingsmart.org/IFC/RELEASE/IFC2x3/TC1/HTML/ifcgeometryresource/lexical/ifcdimensioncount.htm
#[derive(
    Debug, EnumString, VariantNames, Display, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord,
)]
pub enum DimensionCount {
    #[default]
    #[strum(to_string = "1")]
    One,

    #[strum(to_string = "2")]
    Two,

    #[strum(to_string = "3")]
    Three,
}

impl IFCParse for DimensionCount {
    fn parse<'a>() -> impl IFCParser<'a, Self> {
        let variants: [&str; Self::VARIANTS.len()] =
            Self::VARIANTS.try_into().expect("statically known");

        delimited(
            p_space_or_comment(),
            alt(variants
                .map(|v| (v, Self::from_str(v).unwrap()))
                .map(|(k, v)| k.map(move |_| v))),
            p_space_or_comment(),
        )
    }

    fn fallback<'a>() -> impl IFCParser<'a, OptionalParameter<Self>>
    where
        Self: Sized,
    {
        dec_int.map(|_: i32| OptionalParameter::Inherited(Inherited))
    }
}

#[cfg(test)]
mod test {
    use winnow::Parser;

    use crate::parser::{optional::OptionalParameter, IFCParse};

    use super::DimensionCount;

    #[test]
    fn dimension_count() {
        assert_eq!(
            DimensionCount::parse().parse("1").unwrap(),
            DimensionCount::One
        );

        assert_eq!(
            DimensionCount::parse().parse("2").unwrap(),
            DimensionCount::Two
        );

        assert_eq!(
            DimensionCount::parse().parse("3").unwrap(),
            DimensionCount::Three
        );
    }

    #[test]
    fn fallback_dimension_count() {
        let example = "0";

        let parsed: OptionalParameter<DimensionCount> =
            OptionalParameter::parse().parse(example).unwrap();

        assert!(parsed.is_inherited());
    }
}
