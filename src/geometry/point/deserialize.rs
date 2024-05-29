use winnow::{combinator::delimited, Parser};

use super::{Point2D, Point3D};
use crate::parser::{
    geometry::{p_vec2, p_vec3},
    optional::OptionalParse,
    IFCParser,
};

impl OptionalParse for Point2D {
    fn opt_parse<'a>() -> impl IFCParser<'a, Self>
    where
        Self: Sized,
    {
        p_vec2().map(Self)
    }
}

impl Point2D {
    pub fn parse<'a>() -> impl IFCParser<'a, Self> {
        delimited("IFCCARTESIANPOINT(", Self::opt_parse(), ");")
    }
}

impl OptionalParse for Point3D {
    fn opt_parse<'a>() -> impl IFCParser<'a, Self>
    where
        Self: Sized,
    {
        p_vec3().map(Self)
    }
}

impl Point3D {
    pub fn parse<'a>() -> impl IFCParser<'a, Self> {
        delimited("IFCCARTESIANPOINT(", Self::opt_parse(), ");")
    }
}

#[test]
fn parse_point_2d_works() {
    let data = "IFCCARTESIANPOINT((0.,0.));";
    let p = Point2D::parse().parse(data).unwrap();
    println!("{p:?}");
    assert_eq!(p.to_string(), data);
}

#[test]
fn parse_point_3d_works() {
    let data = "IFCCARTESIANPOINT((0.,0.,-0.199999999999868));";
    let p = Point3D::parse().parse(data).unwrap();
    println!("{p:?}");
    assert_eq!(p.to_string(), data);
}
