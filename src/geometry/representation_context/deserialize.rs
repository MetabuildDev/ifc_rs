use winnow::{ascii::dec_uint, combinator::delimited, Parser};

use super::GeometricRepresentationContext;
use crate::{
    id::Id,
    parser::{
        comma::Comma,
        optional::{IFCParse, OptionalParameter},
        IFCParser,
    },
};

impl GeometricRepresentationContext {
    pub fn parse<'a>() -> impl IFCParser<'a, Self> {
        delimited(
            "IFCGEOMETRICREPRESENTATIONCONTEXT(",
            (
                OptionalParameter::parse(),
                Comma::parse(),
                OptionalParameter::parse(),
                Comma::parse(),
                dec_uint,
                Comma::parse(),
                OptionalParameter::parse(),
                Comma::parse(),
                Id::parse(),
                Comma::parse(),
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
    let parsed = GeometricRepresentationContext::parse().parse(data).unwrap();
    assert_eq!(data, parsed.to_string());

    let data = "IFCGEOMETRICREPRESENTATIONCONTEXT('TestIdentifier',$,3,$,#99,$);";
    let parsed = GeometricRepresentationContext::parse().parse(data).unwrap();
    assert_eq!(data, parsed.to_string());
}
