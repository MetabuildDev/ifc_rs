use winnow::{combinator::delimited, Parser};

use super::{Point2D, Point3D};
use crate::parser::{
    geometry::{p_vec2, p_vec3},
    optional::IFCParse,
    IFCParser,
};

impl IFCParse for Point2D {
    fn parse<'a>() -> impl IFCParser<'a, Self>
    where
        Self: Sized,
    {
        delimited("IFCCARTESIANPOINT(", p_vec2().map(Self), ");")
    }
}

impl IFCParse for Point3D {
    fn parse<'a>() -> impl IFCParser<'a, Self>
    where
        Self: Sized,
    {
        delimited("IFCCARTESIANPOINT(", p_vec3().map(Self), ");")
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
