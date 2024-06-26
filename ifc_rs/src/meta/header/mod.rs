mod deserialize;
mod serialize;

use serde::{Deserialize, Serialize};

use super::version;
use crate::prelude::*;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Header {
    pub version: version::Version,
    pub description: description::FileDescription,
    pub name: details::FileDetails,
    pub schema: schema::FileSchemas,
}

impl IfcVerify for Header {}
impl IfcType for Header {}

pub mod description {

    use serde::{Deserialize, Serialize};
    use strum::{Display, EnumString, VariantNames};

    #[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub struct FileDescription {
        pub descriptions: Vec<ViewDefinition>,
        pub implementation_level: ImplementationLevel,
    }

    #[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub struct ViewDefinition {
        pub name: String,
        pub items: Vec<String>,
    }

    #[derive(
        Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Display, EnumString, VariantNames,
    )]
    pub enum ImplementationLevel {
        #[strum(to_string = "2;1")]
        _2_1,
    }
}

pub mod details {
    use chrono::Utc;
    use serde::{Deserialize, Serialize};

    #[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub struct FileDetails {
        pub name: FileName,
        pub timestamp: TimeStamp,
        pub author: Vec<Author>,
        pub organization: Vec<Organization>,
        pub preprocessor_version: PreprocessorVersion,
        pub originating_system: OriginatingSystem,
        pub authorization: Authorization,
    }

    #[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub struct FileName(pub String);
    #[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub struct TimeStamp(pub chrono::DateTime<Utc>);

    impl Default for TimeStamp {
        fn default() -> Self {
            Self(Utc::now())
        }
    }

    #[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub struct Author(pub String);
    #[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub struct Organization(pub String);
    #[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub struct PreprocessorVersion(pub String);
    #[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub struct OriginatingSystem(pub String);
    #[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub struct Authorization(pub String);
}

pub mod schema {
    use serde::{Deserialize, Serialize};
    use strum::{Display, EnumString, VariantNames};

    #[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub struct FileSchemas(pub Vec<FileSchema>);

    #[derive(
        Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Display, EnumString, VariantNames,
    )]
    pub enum FileSchema {
        IFC2X3,
        IFC4X2,
        IFC4,
    }
}

#[cfg(test)]
mod header_tests {

    use chrono::DateTime;

    use crate::meta::version::Version;

    use super::{
        description::{FileDescription, ImplementationLevel, ViewDefinition},
        details::{
            Author, Authorization, FileDetails, FileName, Organization, OriginatingSystem,
            PreprocessorVersion, TimeStamp,
        },
        schema::{FileSchema, FileSchemas},
        Header,
    };

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
            version: Version::ISO_10303_21,
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
}
