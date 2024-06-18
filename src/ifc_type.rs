use std::{any::Any, fmt::Display};

use anyhow::Result;
use downcast_rs::{self, impl_downcast, Downcast};

use crate::IFC;

pub trait IfcVerify: Any {
    fn verify_id_types(&self, _ifc: &IFC) -> Result<()> {
        Ok(())
    }

    fn type_name(&self) -> &str {
        std::any::type_name::<Self>()
    }
}

pub trait IfcType: Downcast + Any + Display + IfcVerify {}
impl_downcast!(IfcType);
