use winnow::{ascii::dec_uint, combinator::delimited, Parser};

use super::GeometricRepresentationContext;
use crate::{
    id::Id,
    parser::{
        optional::{IFCParse, OptionalParameter},
        p_space_or_comment_surrounded, IFCParser,
    },
};

impl GeometricRepresentationContext {
    pub fn parse<'a>() -> impl IFCParser<'a, Self> {
        delimited(
            "IFCGEOMETRICREPRESENTATIONCONTEXT(",
            (
                OptionalParameter::parse(),
                p_space_or_comment_surrounded(","),
                OptionalParameter::parse(),
                p_space_or_comment_surrounded(","),
                dec_uint,
                p_space_or_comment_surrounded(","),
                OptionalParameter::parse(),
                p_space_or_comment_surrounded(","),
                Id::parse(),
                p_space_or_comment_surrounded(","),
                OptionalParameter::parse(),
            ),
            ");",
        )
        .map(
            |(
                context_identifier,
                _,
                context_type,
                _,
                coord_space_dimension,
                _,
                precision,
                _,
                world_coord_system,
                _,
                true_north,
            )| Self {
                context_identifier,
                context_type,
                coord_space_dimension,
                precision,
                world_coord_system,
                true_north,
            },
        )
    }
}

#[test]
fn parse_axis_3d_works() {
    let data = "IFCGEOMETRICREPRESENTATIONCONTEXT($,'Model',3,1.00000000000000E-5,#99,#100);";
    let _ = GeometricRepresentationContext::parse().parse(data).unwrap();
    let data = "IFCGEOMETRICREPRESENTATIONCONTEXT('TestIdentifier',$,3,$,#99,$);";
    let _ = GeometricRepresentationContext::parse().parse(data).unwrap();
    println!("{data}")
}
