mod deserialize;
mod serialize;

use serde::{Deserialize, Serialize};

use crate::ifc_type::{IfcType, IfcVerify};

use super::version::Version;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Footer {
    pub version: Version,
}

impl IfcVerify for Footer {}
impl IfcType for Footer {}

#[test]
fn serde_roundtrips_backwards() {
    let footer = Footer {
        version: Version::ISO_10303_21,
    };
    let footer_str = serde_json::to_string(&footer).unwrap();
    let footer_again: Footer = serde_json::from_str(&footer_str).unwrap();
    assert_eq!(footer, footer_again);
}
