use winnow::Parser;

use crate::{
    geometry::{
        geometric_projection::GeometricProjection,
        representation_subcontext::GeometricRepresentationSubContext,
    },
    id::{Id, TypedId},
    parser::{comma::Comma, optional::OptionalParameter, IFCParse, IFCParser},
};

impl IFCParse for GeometricRepresentationSubContext {
    fn parse<'a>() -> impl IFCParser<'a, Self> {
        winnow::seq! {
            Self {
                _: "IFCGEOMETRICREPRESENTATIONSUBCONTEXT(",
                context_identifier: OptionalParameter::parse(),
                _: Comma::parse(),
                context_type: OptionalParameter::parse(),
                _: Comma::parse(),
                coord_space_dimension: OptionalParameter::parse(),
                _: Comma::parse(),
                precision: OptionalParameter::parse(),
                _: Comma::parse(),
                world_coord_system: OptionalParameter::parse(),
                _: Comma::parse(),
                true_north: OptionalParameter::parse(),
                _: Comma::parse(),
                parent_context: Id::parse().map(TypedId::new),
                _: Comma::parse(),
                target_scale: OptionalParameter::parse(),
                _: Comma::parse(),
                target_view: GeometricProjection::parse(),
                _: Comma::parse(),
                user_defined_target_view: OptionalParameter::parse(),
                _: ");"
            }
        }
    }
}

#[test]
fn parse_geometric_representation_subcontext_works() {
    let data =
        "IFCGEOMETRICREPRESENTATIONSUBCONTEXT('Axis','Model',*,*,*,*,#102,$,.GRAPH_VIEW.,$);";
    let parsed = GeometricRepresentationSubContext::parse()
        .parse(data)
        .unwrap();
    assert_eq!(data, parsed.to_string());

    let data =
        "IFCGEOMETRICREPRESENTATIONSUBCONTEXT('Axis','Model',*,*,*,*,#102,0.01,.USERDEFINED.,'UserLabel');";
    let parsed = GeometricRepresentationSubContext::parse()
        .parse(data)
        .unwrap();
    assert_eq!(data, parsed.to_string());

    let data = "IFCGEOMETRICREPRESENTATIONSUBCONTEXT('Axis','Model',1,$,$,$,#12,$,.MODEL_VIEW.,$);";
    let parsed = GeometricRepresentationSubContext::parse()
        .parse(data)
        .unwrap();
    assert_eq!(data, parsed.to_string());
}
