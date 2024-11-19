pub(crate) mod id_or;
pub(crate) mod ifc_uuid;
pub(crate) mod plain;
pub(crate) mod typed;

pub use id_or::IdOr;
pub use ifc_uuid::IfcGloballyUniqueId;
pub use plain::Id;
pub use typed::TypedId;
