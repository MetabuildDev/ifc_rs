use winnow::{combinator::delimited, Parser};

use super::{Direction2D, Direction3D};
use crate::parser::{
    geometry::{p_vec2, p_vec3},
    optional::IFCParse,
    IFCParser,
};

impl IFCParse for Direction2D {
    fn parse<'a>() -> impl IFCParser<'a, Self>
    where
        Self: Sized,
    {
        delimited("IFCDIRECTION(", p_vec2().map(Self), ");")
    }
}

impl IFCParse for Direction3D {
    fn parse<'a>() -> impl IFCParser<'a, Self>
    where
        Self: Sized,
    {
        delimited("IFCDIRECTION(", p_vec3().map(Self), ");")
    }
}

#[test]
fn parse_direction_2d_works() {
    let data = "IFCDIRECTION((6.12303176911189E-17,1.));";
    let p = Direction2D::parse().parse(data).unwrap();
    println!("{p:?}");
    assert_eq!(p.to_string(), data);
}

#[test]
fn parse_direction_3d_works() {
    let data = "IFCDIRECTION((0.,0.,-1.));";
    let p = Direction3D::parse().parse(data).unwrap();
    println!("{p:?}");
    assert_eq!(p.to_string(), data);
}
