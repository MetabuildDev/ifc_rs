use winnow::{
    combinator::{delimited, separated_pair},
    Parser,
};

use super::{Axis2D, Axis3D};
use crate::{
    id::TypedId,
    parser::{comma::Comma, optional::OptionalParameter, IFCParse, IFCParser},
};

impl IFCParse for Axis2D {
    fn parse<'a>() -> impl IFCParser<'a, Self> {
        delimited(
            "IFCAXIS2PLACEMENT2D(",
            separated_pair(TypedId::parse(), Comma::parse(), OptionalParameter::parse()),
            ");",
        )
        .map(|(location, local_x)| Self { location, local_x })
    }
}

impl IFCParse for Axis3D {
    fn parse<'a>() -> impl IFCParser<'a, Self> {
        delimited(
            "IFCAXIS2PLACEMENT3D(",
            (
                TypedId::parse(),
                Comma::parse(),
                OptionalParameter::parse(),
                Comma::parse(),
                OptionalParameter::parse(),
            ),
            ");",
        )
        .map(|(location, _, local_z, _, local_x)| Self {
            location,
            local_z,
            local_x,
        })
    }
}

#[test]
fn parse_axis_2d_works() {
    let data = "IFCAXIS2PLACEMENT2D(#248,#23);";
    let _ = Axis2D::parse().parse(data).unwrap();
    let data = "IFCAXIS2PLACEMENT2D(#248,$);";
    let a = Axis2D::parse().parse(data).unwrap();
    println!("{a}")
}

#[test]
fn parse_axis_3d_works() {
    let data = "IFCAXIS2PLACEMENT3D(#6,$,$);";
    let _ = Axis3D::parse().parse(data).unwrap();
    let data = "IFCAXIS2PLACEMENT3D(#6,#12,$);";
    let _ = Axis3D::parse().parse(data).unwrap();
    let data = "IFCAXIS2PLACEMENT3D(#6,$,#23);";
    let _ = Axis3D::parse().parse(data).unwrap();
    let data = "IFCAXIS2PLACEMENT3D(#6,#53,#23);";
    let a = Axis3D::parse().parse(data).unwrap();
    println!("{a}")
}
