use winnow::{combinator::delimited, Parser};

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
        delimited(
            "IFCGEOMETRICREPRESENTATIONSUBCONTEXT(",
            (
                OptionalParameter::parse(),
                Comma::parse(),
                OptionalParameter::parse(),
                Comma::parse(),
                OptionalParameter::parse(),
                Comma::parse(),
                OptionalParameter::parse(),
                Comma::parse(),
                OptionalParameter::parse(),
                Comma::parse(),
                OptionalParameter::parse(),
                Comma::parse(),
                Id::parse(),
                Comma::parse(),
                OptionalParameter::parse(),
                Comma::parse(),
                GeometricProjection::parse(),
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
                _,
                parent_context,
                _,
                target_scale,
                _,
                target_view,
                _,
                user_defined_target_view,
            )| Self {
                context_identifier,
                context_type,
                coord_space_dimension,
                precision,
                world_coord_system,
                true_north,
                parent_context: TypedId::new(parent_context),
                target_scale,
                target_view,
                user_defined_target_view,
            },
        )
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

    "IFCGEOMETRICREPRESENTATIONSUBCONTEXT('Axis','Model',0,$,$,$,#12,$,.MODEL_VIEW.,$);";
    GeometricRepresentationSubContext::parse()
        .parse(data)
        .unwrap();
}
