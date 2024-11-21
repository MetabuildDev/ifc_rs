use super::GeometricRepresentationContext;
use crate::{
    geometry::dimension_count::DimensionCount,
    id::Id,
    parser::{comma::Comma, optional::OptionalParameter, IFCParse, IFCParser},
};

impl IFCParse for GeometricRepresentationContext {
    fn parse<'a>() -> impl IFCParser<'a, Self> {
        winnow::seq! {
            Self {
                _: "IFCGEOMETRICREPRESENTATIONCONTEXT(",
                context_identifier: OptionalParameter::parse(),
                _: Comma::parse(),
                context_type: OptionalParameter::parse(),
                _: Comma::parse(),
                coord_space_dimension: DimensionCount::parse(),
                _: Comma::parse(),
                precision: OptionalParameter::parse(),
                _: Comma::parse(),
                world_coord_system: Id::parse(),
                _: Comma::parse(),
                true_north: OptionalParameter::parse(),
                _: ");",
            }
        }
    }
}

#[test]
fn parse_geometric_representation_context_works() {
    use winnow::Parser;

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
