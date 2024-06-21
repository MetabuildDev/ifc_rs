#![allow(dead_code)]

use anyhow::{anyhow, Context, Result};
use parser::IFCParse;
use std::{fmt::Display, fs, path::Path};
use winnow::{seq, Parser};

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

pub mod geometry;
pub mod id;
pub mod ifc_builder;
pub mod ifc_extractor;
pub mod ifc_type;
pub mod material;
pub mod meta;
pub mod objects;
pub mod parser;
pub mod prelude;
pub mod relations;
pub mod traits;
pub mod units;

pub struct IFC {
    pub header: Header,

    pub data: DataMap,

    pub footer: Footer,
}

impl IFC {
    pub fn from_file(path: impl AsRef<Path>) -> Result<Self> {
        let contents = fs::read_to_string(path)?;
        let s = contents.as_str();

        Self::from_str(s)
    }

    pub fn from_str(mut s: &str) -> Result<Self> {
        let me = seq!(Self {
            header: Header::parse(),
            data: DataMap::parse(),
            footer: Footer::parse(),
        })
        .parse_next(&mut s)
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
                schema: FileSchemas(vec![FileSchema::IFC4x2]),
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
