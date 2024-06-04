pub mod deserialize;
mod serialize;

use std::collections::BTreeMap;

use strum::Display;

use crate::id::Id;

/// CRITICAL: split up the index map into a proper struct with fields which hold Hashmaps mapping
/// indices to one specific type instead of an enum
pub struct DataMap(pub BTreeMap<Id, DataValue>);

#[derive(Debug, Clone, Display)]
pub enum DataValue {
    #[strum(to_string = "{s}")]
    Any { s: String },
}
