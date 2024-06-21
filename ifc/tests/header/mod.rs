use chrono::DateTime;
use ifc::meta::header::description::{FileDescription, ImplementationLevel, ViewDefinition};
use ifc::meta::header::details::{
    Author, Authorization, FileDetails, FileName, Organization, OriginatingSystem,
    PreprocessorVersion, TimeStamp,
};
use ifc::meta::header::schema::{FileSchema, FileSchemas};
use ifc::meta::header::Header;

use crate::common::example_version;

fn example_description() -> FileDescription {
    FileDescription {
        descriptions: vec![ViewDefinition {
            name: String::from("Test file description"),
            items: vec![String::from("Test file description item")],
        }],
        implementation_level: ImplementationLevel::_2_1,
    }
}

fn example_filedetails() -> FileDetails {
    FileDetails {
        name: FileName(String::from("Test filename")),
        timestamp: TimeStamp(DateTime::default()),
        author: vec![Author(String::from("Test author"))],
        organization: vec![Organization(String::from("Test organization"))],
        preprocessor_version: PreprocessorVersion(String::from("Test preprocessor version")),
        originating_system: OriginatingSystem(String::from("Test originating system")),
        authorization: Authorization(String::from("Test authorization")),
    }
}

fn example_schema() -> FileSchemas {
    FileSchemas(vec![FileSchema::IFC2X3])
}

fn example_header() -> Header {
    Header {
        version: example_version(),
        description: example_description(),
        name: example_filedetails(),
        schema: example_schema(),
    }
}

#[test]
fn serde_roundtrips_backwards() {
    let header = example_header();
    let header_str = serde_json::to_string(&header).unwrap();
    let header_again: Header = serde_json::from_str(&header_str).unwrap();
    assert_eq!(header, header_again);
}
