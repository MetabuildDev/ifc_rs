mod deserialize;
mod serialize;

use serde::{Deserialize, Serialize};

use super::version::Version;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Footer {
    pub version: Version,
}
