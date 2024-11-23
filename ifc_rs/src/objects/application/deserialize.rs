use crate::{
    id::TypedId,
    objects::application::Application,
    parser::{comma::Comma, string::StringPrimitive, IFCParse, IFCParser},
};

impl IFCParse for Application {
    fn parse<'a>() -> impl IFCParser<'a, Self> {
        winnow::seq! {
            Application {
                _ : "IFCAPPLICATION(",
                application_developer: TypedId::parse(),
                _: Comma::parse(),
                version: StringPrimitive::parse(),
                _: Comma::parse(),
                application_full_name: StringPrimitive::parse(),
                _: Comma::parse(),
                application_identifier: StringPrimitive::parse(),
                _: ");",
            }
        }
    }
}

#[test]
fn parse_application_works() {
    use winnow::Parser;
    let data = "IFCAPPLICATION(#9,'0.0.1.0','ggRhinoIFC - Geometry Gym Plug-in for Rhino3d','ggRhinoIFC');";
    let parsed = Application::parse().parse(data).unwrap();
    assert_eq!(data, parsed.to_string());
}
