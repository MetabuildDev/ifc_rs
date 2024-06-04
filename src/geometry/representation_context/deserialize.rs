use winnow::{combinator::delimited, Parser};

use super::GeometricRepresentationContext;
use crate::{
    geometry::dimension_count::DimensionCount,
    id::Id,
    parser::{comma::Comma, optional::OptionalParameter, IFCParse, IFCParser},
};

impl IFCParse for GeometricRepresentationContext {
    fn parse<'a>() -> impl IFCParser<'a, Self> {
        delimited(
            "IFCGEOMETRICREPRESENTATIONCONTEXT(",
            (
                OptionalParameter::parse(),
                Comma::parse(),
                OptionalParameter::parse(),
                Comma::parse(),
                DimensionCount::parse(),
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
fn parse_geometric_representation_context_works() {
    let data = "IFCGEOMETRICREPRESENTATIONCONTEXT($,'Model',3,0.00001,#99,#100);";
    let parsed = GeometricRepresentationContext::parse().parse(data).unwrap();
    assert_eq!(data, parsed.to_string());

    let data = "IFCGEOMETRICREPRESENTATIONCONTEXT('TestIdentifier',$,3,$,#99,$);";
    let parsed = GeometricRepresentationContext::parse().parse(data).unwrap();
    assert_eq!(data, parsed.to_string());

    // invalid dimension count (4)
    let data = "IFCGEOMETRICREPRESENTATIONCONTEXT($,'Model',4,0.00001,#99,#100);";
    assert!(GeometricRepresentationContext::parse().parse(data).is_err());
}
