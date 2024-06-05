use std::str::FromStr;

use strum::{Display, EnumString, VariantNames};
use winnow::combinator::{alt, delimited};
use winnow::Parser;

use crate::parser::*;

#[derive(EnumString, VariantNames, Display, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum IfcBool {
    #[strum(to_string = ".TRUE.")]
    #[strum(serialize = ".T.")]
    True,

    #[strum(to_string = ".FALSE.")]
    #[strum(serialize = ".F.")]
    False,

    #[strum(to_string = ".UNKNOWN.")]
    #[strum(serialize = ".U.")]
    Unknown,
}

impl IFCParse for IfcBool {
    fn parse<'a>() -> impl IFCParser<'a, Self> {
        let variants: [&str; Self::VARIANTS.len()] =
            Self::VARIANTS.try_into().expect("statically known");

        delimited(
            p_space_or_comment(),
            alt(variants
                .map(|v| (v, Self::from_str(v).expect("valid IfcBool")))
                .map(|(k, v)| k.map(move |_| v))),
            p_space_or_comment(),
        )
    }
}

#[cfg(test)]
mod test {
    use winnow::Parser;

    use super::IfcBool;
    use crate::parser::IFCParse;

    #[test]
    fn ifc_bool_test_full() {
        assert_eq!(IfcBool::True, IfcBool::parse().parse(".TRUE.").unwrap());
        assert_eq!(IfcBool::False, IfcBool::parse().parse(".FALSE.").unwrap());
        assert_eq!(
            IfcBool::Unknown,
            IfcBool::parse().parse(".UNKNOWN.").unwrap()
        );
    }

    #[test]
    fn ifc_bool_test_abbreviated() {
        assert_eq!(IfcBool::True, IfcBool::parse().parse(".T.").unwrap());
        assert_eq!(IfcBool::False, IfcBool::parse().parse(".F.").unwrap());
        assert_eq!(IfcBool::Unknown, IfcBool::parse().parse(".U.").unwrap());
    }

    #[test]
    fn ifc_bool_serialize() {
        assert_eq!(IfcBool::True.to_string(), ".TRUE.");
        assert_eq!(IfcBool::False.to_string(), ".FALSE.");
        assert_eq!(IfcBool::Unknown.to_string(), ".UNKNOWN.");
    }
}
