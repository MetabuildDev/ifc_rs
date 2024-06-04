use winnow::{combinator::delimited, Parser};

use crate::{
    id::Id,
    objects::application::Application,
    parser::{comma::Comma, label::Label, IFCParse, IFCParser},
};

impl IFCParse for Application {
    fn parse<'a>() -> impl IFCParser<'a, Self> {
        delimited(
            "IFCAPPLICATION(",
            (
                Id::parse(),
                Comma::parse(),
                Label::parse(),
                Comma::parse(),
                Label::parse(),
                Comma::parse(),
                Label::parse(),
            ),
            ");",
        )
        .map(
            |(
                application_developer,
                _,
                version,
                _,
                application_full_name,
                _,
                application_identifier,
            )| Self {
                application_developer,
                version,
                application_full_name,
                application_identifier,
            },
        )
    }
}

#[test]
fn parse_application_works() {
    let data = "IFCAPPLICATION(#9,'0.0.1.0','ggRhinoIFC - Geometry Gym Plug-in for Rhino3d','ggRhinoIFC');";
    let parsed = Application::parse().parse(data).unwrap();
    assert_eq!(data, parsed.to_string());
}
