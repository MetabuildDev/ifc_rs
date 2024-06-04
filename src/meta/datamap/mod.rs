pub mod deserialize;
mod serialize;

use std::{any::Any, collections::BTreeMap};

use anyhow::{anyhow, Result};
use strum::Display;
use winnow::{combinator::alt, Parser};

use crate::{
    geometry::Geometry,
    id::Id,
    objects::{wall::Wall, walltype::WallType, Objects},
    parser::optional::IFCParse,
    units::Units,
};

/// CRITICAL: split up the index map into a proper struct with fields which hold Hashmaps mapping
/// indices to one specific type instead of an enum
pub struct DataMap(pub BTreeMap<Id, DataValue>);

pub struct ParsedMap(pub BTreeMap<Id, Box<dyn Any>>);

#[derive(Debug, Clone, Display)]
pub enum DataValue {
    #[strum(to_string = "{s}")]
    Any { s: String },
}

impl DataValue {
    pub fn parse_types(&self) -> Result<Box<dyn Any>> {
        let mut s = match self {
            DataValue::Any { s } => s.clone(),
        };

        alt((Objects::parse(), Geometry::parse(), Units::parse()))
            .parse_next(&mut s)
            .map_err(|err| anyhow!("content parsing failed: {err:#?}"))
    }
}
