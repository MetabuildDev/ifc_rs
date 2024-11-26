#![allow(clippy::too_many_arguments)]
#![allow(dead_code)]

use anyhow::{anyhow, Context, Result};
use parser::IFCParse;
use std::{fmt::Display, fs, path::Path, str::FromStr};
use winnow::Parser;

use meta::{
    datamap::DataMap,
    footer::Footer,
    header::{
        description::{FileDescription, ImplementationLevel},
        details::FileDetails,
        schema::{FileSchema, FileSchemas},
        Header,
    },
    version::Version,
};

/// module containing all the IFC objects related to the general geometry of the model
pub mod geometry;
/// module that defines IDs as they're used within the STEP IFC format. This also includes some
/// utilities such as typed IDs or objects which can both be specified by ID or in-place values
pub mod id;
/// module that defines builders to easily create valid IFC files without much knowledge about how
/// the IFC types are structured
pub mod ifc_builder;
/// module that defines utilities to query properties of an in-memory IFC file in the format of
/// this crate
pub mod ifc_extractor;
/// module containing definitions of materials which are used to define the look and other
/// properties of IFC elements like walls, slabs, windows, etc.
pub mod materials;
/// module containing definitions of the structure of the meta information that comes with IFC
/// files in STEP format
pub mod meta;
/// module containing definitions of the actual IFC Objects
pub mod objects;
/// module containing parsing trait and utilities used to deserialize the IFC STEP format into rust
/// types
pub mod parser;
/// common prelude module for the whole crate
pub mod prelude;
/// module containing property set definition and properties which can be used to enhance elements
/// with extra data
pub mod properties;
/// module containing definitions of so called IFC relation objects which link one-or-more IFC
/// objects to one-or-more other IFC objects or properties
pub mod relations;
/// general module containing definitions of traits used throughout the crate
pub mod traits;
/// module containing definitions of units for measurements and property definitions
pub mod units;
/// module containing definitions of primitive values which are used in properties
pub mod values;

/// Central IFC Object which holds the information about the whole model together with meta
/// information about the model
pub struct IFC {
    /// the header holds most meta information
    pub header: Header,

    /// the data map contains all IFC object associated to a model internal ID
    pub data: DataMap,

    /// the footer with meta information
    pub footer: Footer,
}

impl IFC {
    /// loads an IFC file from the given path
    ///
    /// This may fail if the file doesn't exist or if the parsing fails
    pub fn from_file(path: impl AsRef<Path>) -> Result<Self> {
        let contents = fs::read_to_string(path)?;
        let s = contents.as_str();

        Self::from_str(s)
    }
}

impl IFCParse for IFC {
    fn parse<'a>() -> impl parser::IFCParser<'a, Self> {
        winnow::seq! {
            IFC {
                header: Header::parse(),
                data: DataMap::parse(),
                footer: Footer::parse(),
            }
        }
    }
}

impl FromStr for IFC {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        let me = IFC::parse()
            .parse(s)
            .map_err(|err| anyhow!("parsing failed: {err:#?}"))?;

        for (id, ifc_type) in me.data.0.iter() {
            ifc_type.verify_id_types(&me).context(format!("ID: {id}"))?;
        }

        Ok(me)
    }
}

impl Default for IFC {
    fn default() -> Self {
        Self {
            header: Header {
                version: Version::ISO_10303_21,
                description: FileDescription {
                    descriptions: Vec::new(),
                    implementation_level: ImplementationLevel::_2_1,
                },
                name: FileDetails::default(),
                schema: FileSchemas(vec![FileSchema::IFC4X3_ADD2]),
            },
            data: Default::default(),
            footer: Footer {
                version: Version::ISO_10303_21,
            },
        }
    }
}

impl Display for IFC {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}\n{}\n{}", self.header, self.data, self.footer)
    }
}

#[cfg(test)]
mod test {
    use crate::objects::wall::test::print_wall_hierarchy;

    use super::IFC;
    use anyhow::Result;

    #[test]
    fn load_wall_example_file() -> Result<()> {
        let ifc = IFC::from_file("resources/wall-standard-case.ifc")?;

        print_wall_hierarchy(&ifc);

        Ok(())
    }

    #[test]
    fn load_archicad_file() -> Result<()> {
        let ifc = IFC::from_file("resources/AC20-FZK-Haus.ifc")?;

        print_wall_hierarchy(&ifc);

        Ok(())
    }
}
