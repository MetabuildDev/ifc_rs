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
