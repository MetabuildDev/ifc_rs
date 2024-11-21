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

pub(crate) mod description {

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

pub(crate) mod details {
    use chrono::Utc;
    use serde::{Deserialize, Serialize};

    #[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub struct FileDetails {
        pub name: FileName,
        pub timestamp: TimeStamp,
        pub author: AuthorList,
        pub organization: OrganizationList,
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

    #[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub struct AuthorList(pub Vec<Author>);

    impl Default for AuthorList {
        fn default() -> Self {
            Self(vec![Author(String::from(""))])
        }
    }

    impl std::ops::Deref for AuthorList {
        type Target = Vec<Author>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }

    impl std::ops::DerefMut for AuthorList {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }

    #[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub struct OrganizationList(pub Vec<Organization>);

    impl Default for OrganizationList {
        fn default() -> Self {
            Self(vec![Organization(String::from(""))])
        }
    }

    impl std::ops::Deref for OrganizationList {
        type Target = Vec<Organization>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }

    impl std::ops::DerefMut for OrganizationList {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
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

pub(crate) mod schema {
    use serde::{Deserialize, Serialize};
    use strum::{Display, EnumString, VariantNames};

    #[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub struct FileSchemas(pub Vec<FileSchema>);

    #[allow(non_camel_case_types)]
    /// Don't ask me what ADD2 is supposed to be, I used the official ifc validator
    ///
    /// https://validate.buildingsmart.org/
    ///
    /// and got this as an error message:
    ///
    /// "
    /// Description: Only official IFC versions allowed ,
    /// Expected: {"oneOf": ["IFC4X3_ADD2", "IFC4", "IFC2X3"]}
    /// "
    ///
    /// after which I adjusted this enum here
    #[derive(
        Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Display, EnumString, VariantNames,
    )]
    pub enum FileSchema {
        IFC4X3_ADD2,
        IFC4,
        IFC2X3,
    }
}

#[cfg(test)]
mod header_tests {

    use chrono::DateTime;

    use crate::meta::version::Version;

    use super::{
        description::{FileDescription, ImplementationLevel, ViewDefinition},
        details::{
            Author, AuthorList, Authorization, FileDetails, FileName, Organization,
            OrganizationList, OriginatingSystem, PreprocessorVersion, TimeStamp,
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
            author: AuthorList(vec![Author(String::from("Test author"))]),
            organization: OrganizationList(vec![Organization(String::from("Test organization"))]),
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
