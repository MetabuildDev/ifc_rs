use std::fmt::Display;

use chrono::Utc;
use winnow::ascii::dec_int;
use winnow::Parser;

use crate::parser::{IFCParse, IFCParser};

/// An indication of date and time by measuring the number of seconds which
/// have elapsed since the beginning of the year 1970.
///
/// https://standards.buildingsmart.org/IFC/RELEASE/IFC2x3/TC1/HTML/ifcmeasureresource/lexical/ifctimestamp.htm
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct IfcTimestamp(pub chrono::DateTime<Utc>);

impl IFCParse for IfcTimestamp {
    fn parse<'a>() -> impl IFCParser<'a, Self>
    where
        Self: Sized,
    {
        dec_int.map(|i| Self(chrono::DateTime::from_timestamp(i, 0).unwrap()))
    }
}

impl Display for IfcTimestamp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0.timestamp())
    }
}
