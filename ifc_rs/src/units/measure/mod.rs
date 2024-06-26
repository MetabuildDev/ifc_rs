pub mod plane_angle;

pub use plane_angle::PlaneAngleMeasure;

use super::{IFCParse, IFCParser};
use crate::prelude::*;

pub struct Measures;

impl Measures {
    pub fn parse<'a>() -> impl IFCParser<'a, Box<dyn IfcType>> {
        // alt((PlaneAngleMeasure::parse_any()))
        PlaneAngleMeasure::parse_any()
    }
}
