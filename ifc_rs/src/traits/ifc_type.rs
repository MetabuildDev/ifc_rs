use std::{any::Any, fmt::Display};

use anyhow::Result;
use downcast_rs::{self, impl_downcast, Downcast};

use crate::prelude::*;

/// Trait which is mostly automatically implemented via the `ifc_rs_verify_derive` crate. It is used
/// to verify that objects referencing other objects by ID are of a type that fits the purpose. For
/// more information, please visit the docs of the `ifc_rs_verify_derive` crate
pub trait IfcVerify: Any + Send + Sync + 'static {
    fn verify_id_types(&self, _ifc: &IFC) -> Result<()> {
        Ok(())
    }

    fn type_name(&self) -> &str {
        std::any::type_name::<Self>()
    }
}

/// Trait which marks all types that are IFC Objects in a sense that they can be used in the
/// [`crate::IFC`] data map as a value
pub trait IfcType: Downcast + Display + IfcVerify {
    fn to_structure(&self) -> Option<&dyn Structure> {
        None
    }
}
impl_downcast!(IfcType);
