#![allow(non_camel_case_types)]
use serde::{Deserialize, Serialize};
use strum::{Display, EnumString, VariantNames};

#[derive(
    Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Display, EnumString, VariantNames,
)]
pub enum Version {
    #[strum(to_string = "ISO-10303-21")]
    ISO_10303_21,
}
