use winnow::{
    combinator::{delimited, separated_pair},
    Parser,
};

use super::{Axis2D, Axis3D};
use crate::{
    id::Id,
    parser::{
        optional::{IFCParse, OptionalParameter},
        p_space_or_comment_surrounded, IFCParser,
    },
};

impl Axis2D {
    pub fn parse<'a>() -> impl IFCParser<'a, Self> {
        delimited(
            "IFCAXIS2PLACEMENT2D(",
            separated_pair(
                Id::parse(),
                p_space_or_comment_surrounded(","),
                OptionalParameter::parse(),
            ),
            ");",
        )
        .map(|(location, local_x)| Self { location, local_x })
    }
}

impl Axis3D {
    pub fn parse<'a>() -> impl IFCParser<'a, Self> {
        delimited(
            "IFCAXIS2PLACEMENT3D(",
            (
                Id::parse(),
                p_space_or_comment_surrounded(","),
                OptionalParameter::parse(),
                p_space_or_comment_surrounded(","),
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
