use winnow::combinator::alt;

use crate::{parser::*, prelude::*};

pub(crate) mod base;
pub(crate) mod extended_base;
pub(crate) mod material;
pub(crate) mod prelude;
pub(crate) mod set;
pub(crate) mod single_value;

pub struct Properties;

impl Properties {
    pub fn parse<'a>() -> impl IFCParser<'a, Box<dyn IfcType>> {
        alt((
            material::MaterialProperties::parse_any(),
            set::PropertySet::parse_any(),
            single_value::PropertySingleValue::parse_any(),
        ))
    }
}
