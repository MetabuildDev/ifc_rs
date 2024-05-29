use std::str::FromStr;

use strum::{Display, EnumString, VariantNames};
use winnow::combinator::{alt, delimited};
use winnow::Parser;

use crate::parser::optional::OptionalParse;
use crate::parser::*;

#[derive(EnumString, VariantNames, Display, Clone, Copy)]
pub enum IfcPrefix {
    // 10^18
    #[strum(to_string = ".EXA.")]
    Exa,

    // 10^15
    #[strum(to_string = ".PETA.")]
    Peta,

    // 10^12
    #[strum(to_string = ".TERA.")]
    Tera,

    // 10^9
    #[strum(to_string = ".GIGA.")]
    Giga,

    // 10^6
    #[strum(to_string = ".MEGA.")]
    Mega,

    // 10^3
    #[strum(to_string = ".KILO.")]
    Kilo,

    // 10
    #[strum(to_string = ".DECA.")]
    Deca,

    // 10^-1
    #[strum(to_string = ".DECI.")]
    Deci,

    // 10^-2
    #[strum(to_string = ".CENTI.")]
    Centi,

    // 10^-3
    #[strum(to_string = ".MILLI.")]
    Milli,

    // 10^-6
    #[strum(to_string = ".MICRO.")]
    Micro,

    // 10^-9
    #[strum(to_string = ".NANO.")]
    Nano,

    // 10^-12
    #[strum(to_string = ".PICO.")]
    Pico,

    // 10^-15
    #[strum(to_string = ".FEMTO.")]
    Femto,

    // 10^-18
    #[strum(to_string = ".ATTO.")]
    Atto,
}

impl OptionalParse for IfcPrefix {
    fn opt_parse<'a>() -> impl IFCParser<'a, Self> {
        let variants: [&str; Self::VARIANTS.len()] =
            Self::VARIANTS.try_into().expect("statically known");

        delimited(
            p_space_or_comment(),
            alt(variants
                .map(|v| (v, Self::from_str(v).expect("valid IfcPrefix")))
                .map(|(k, v)| k.map(move |_| v))),
            p_space_or_comment(),
        )
    }
}
