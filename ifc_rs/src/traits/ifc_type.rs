use std::{any::Any, fmt::Display};

use anyhow::Result;
use downcast_rs::{self, impl_downcast, Downcast};

use crate::{objects::Structure, IFC};

pub trait IfcVerify: Any + Send + Sync + 'static {
    fn verify_id_types(&self, _ifc: &IFC) -> Result<()> {
        Ok(())
    }

    fn type_name(&self) -> &str {
        std::any::type_name::<Self>()
    }
}

pub trait IfcType: Downcast + Display + IfcVerify {
    fn to_structure(&self) -> Option<&dyn Structure> {
        None
    }
}
impl_downcast!(IfcType);
