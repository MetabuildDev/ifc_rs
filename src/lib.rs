#![allow(dead_code)]

use anyhow::{anyhow, Result};
use parser::IFCParse;
use std::{fs, path::Path};
use winnow::{seq, Parser};

use meta::{
    datamap::DataMap,
    footer::Footer,
    header::{
        description::{FileDescription, ImplementationLevel},
        details::FileDetails,
        schema::FileSchemas,
        Header,
    },
    version::Version,
};

pub mod geometry;
pub mod id;
pub mod ifc_type;
pub mod material;
pub mod meta;
pub mod objects;
pub mod parser;
pub mod relations;
pub mod units;

pub struct IFC {
    pub header: Header,

    pub data: DataMap,

    pub footer: Footer,
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
                schema: FileSchemas(Vec::new()),
            },
            data: Default::default(),
            footer: Footer {
                version: Version::ISO_10303_21,
            },
        }
    }
}

impl IFC {
    pub fn from_file(path: impl AsRef<Path>) -> Result<Self> {
        let contents = fs::read_to_string(path)?;
        let mut s = contents.as_str();

        let me = seq!(Self {
            header: Header::parse(),
            data: DataMap::parse(),
            footer: Footer::parse(),
        })
        .parse_next(&mut s)
        .map_err(|err| anyhow!("parsing failed: {err:#?}"))?;

        Ok(me)
    }
}

#[cfg(test)]
mod test {
    use crate::objects::wall::test::print_wall_hierarchy;

    use super::IFC;
    use anyhow::Result;

    #[test]
    fn load_file() -> Result<()> {
        let ifc = IFC::from_file("resources/wall-standard-case.ifc")?;

        print_wall_hierarchy(&ifc);

        Ok(())
    }
}
