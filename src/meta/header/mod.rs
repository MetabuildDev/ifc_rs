mod deserialize;
mod serialize;

use serde::{Deserialize, Serialize};

use super::version;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Header {
    pub version: version::Version,
    pub description: description::FileDescription,
    pub name: details::FileDetails,
    pub schema: schema::FileSchemas,
}

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

    #[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub struct FileDetails {
        pub name: FileName,
        pub timestamp: TimeStamp,
        pub author: Vec<Author>,
        pub organization: Vec<Organization>,
        pub preprocessor_version: PreprocessorVersion,
        pub originating_system: OriginatingSystem,
        pub authorization: Authorization,
    }

    #[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub struct FileName(pub String);
    #[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub struct TimeStamp(pub chrono::DateTime<Utc>);
    #[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub struct Author(pub String);
    #[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub struct Organization(pub String);
    #[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub struct PreprocessorVersion(pub String);
    #[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub struct OriginatingSystem(pub String);
    #[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
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
        // oof, fix issue where case sensivity matters
        IFC4x2,
    }
}
