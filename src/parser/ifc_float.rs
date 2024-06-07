use std::fmt::Display;
use std::ops::Deref;

use glam::{DVec2, DVec3};
use winnow::ascii::float;
use winnow::Parser;

use crate::parser::geometry::{p_vec2, p_vec3};
use crate::parser::{IFCParse, IFCParser};

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct IfcFloat(pub f64);

impl IFCParse for IfcFloat {
    fn parse<'a>() -> impl IFCParser<'a, Self>
    where
        Self: Sized,
    {
        float.map(Self)
    }
}

impl Display for IfcFloat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", format_double(self.0))
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct IfcDVec2(pub(crate) DVec2);

impl IFCParse for IfcDVec2 {
    fn parse<'a>() -> impl IFCParser<'a, Self>
    where
        Self: Sized,
    {
        p_vec2().map(Self)
    }
}

impl Display for IfcDVec2 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "({x},{y})",
            x = format_double(self.0.x),
            y = format_double(self.0.y),
        )
    }
}

impl Deref for IfcDVec2 {
    type Target = DVec2;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct IfcDVec3(pub(crate) DVec3);

impl IFCParse for IfcDVec3 {
    fn parse<'a>() -> impl IFCParser<'a, Self>
    where
        Self: Sized,
    {
        p_vec3().map(Self)
    }
}

impl Display for IfcDVec3 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "({x},{y},{z})",
            x = format_double(self.0.x),
            y = format_double(self.0.y),
            z = format_double(self.0.z),
        )
    }
}

impl Deref for IfcDVec3 {
    type Target = DVec3;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

pub fn format_double(d: f64) -> String {
    // might need tuning 10 decimals allowed
    let is_scientific = d
        .fract()
        .to_string()
        .chars()
        .filter(|c| c.is_digit(10) && *c != '0')
        .count()
        > 10;

    if is_scientific {
        format!("{0:.1$E}", d, 14)
    } else {
        format!(
            "{d}{opt_p}",
            opt_p = (d.fract() == 0.0).then_some(".").unwrap_or_default()
        )
    }
}
