use std::str::FromStr;

use winnow::combinator::{alt, delimited};
use winnow::Parser;

use crate::parser::*;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum BoolPrimitive {
    True,
    False,
    Unknown,
}

impl BoolPrimitive {
    const VARIANTS: &'static [&'static str] =
        &[".TRUE.", ".FALSE.", ".UNKNOWN.", ".T.", ".F.", ".U."];
}

impl From<bool> for BoolPrimitive {
    fn from(value: bool) -> Self {
        if value {
            Self::True
        } else {
            Self::False
        }
    }
}

impl FromStr for BoolPrimitive {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            ".TRUE." | ".T." => Ok(Self::True),
            ".FALSE." | ".F." => Ok(Self::False),
            ".UNKNOWN." | ".U." => Ok(Self::Unknown),

            _ => Err(format!("failed parsing IfcBool from {s}")),
        }
    }
}

impl Display for BoolPrimitive {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                BoolPrimitive::True => ".TRUE.",
                BoolPrimitive::False => ".FALSE.",
                BoolPrimitive::Unknown => ".UNKNOWN.",
            }
        )
    }
}

impl IFCParse for BoolPrimitive {
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

    use super::BoolPrimitive;
    use crate::parser::IFCParse;

    #[test]
    fn ifc_bool_test_full() {
        assert_eq!(
            BoolPrimitive::True,
            BoolPrimitive::parse().parse(".TRUE.").unwrap()
        );
        assert_eq!(
            BoolPrimitive::False,
            BoolPrimitive::parse().parse(".FALSE.").unwrap()
        );
        assert_eq!(
            BoolPrimitive::Unknown,
            BoolPrimitive::parse().parse(".UNKNOWN.").unwrap()
        );
    }

    #[test]
    fn ifc_bool_test_abbreviated() {
        assert_eq!(
            BoolPrimitive::True,
            BoolPrimitive::parse().parse(".T.").unwrap()
        );
        assert_eq!(
            BoolPrimitive::False,
            BoolPrimitive::parse().parse(".F.").unwrap()
        );
        assert_eq!(
            BoolPrimitive::Unknown,
            BoolPrimitive::parse().parse(".U.").unwrap()
        );
    }

    #[test]
    fn ifc_bool_serialize() {
        assert_eq!(BoolPrimitive::True.to_string(), ".TRUE.");
        assert_eq!(BoolPrimitive::False.to_string(), ".FALSE.");
        assert_eq!(BoolPrimitive::Unknown.to_string(), ".UNKNOWN.");
    }
}
