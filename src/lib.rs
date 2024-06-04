#![allow(dead_code)]

use parser::optional::IFCParse;
use std::{fs, path::Path};
use winnow::{seq, Parser};

use meta::{
    datamap::{deserialize::p_index_map, DataMap},
    footer::Footer,
    header::Header,
};

pub mod geometry;
pub mod id;
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

impl IFC {
    pub fn from_file(path: impl AsRef<Path>) -> Self {
        let contents = fs::read_to_string(path).expect("can't read file");
        let mut s = contents.as_str();

        let me = seq!(Self {
            header: Header::parse(),
            data: p_index_map(),
            footer: Footer::parse(),
        })
        .parse_next(&mut s)
        .unwrap();

        me
    }
}

#[cfg(test)]
mod test {
    use super::IFC;

    #[test]
    fn load_file() {
        IFC::from_file("resources/wall-standard-case.ifc");
    }
}
