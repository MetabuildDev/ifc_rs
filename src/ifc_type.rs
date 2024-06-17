use std::{any::Any, fmt::Display};

use downcast_rs::{self, impl_downcast, Downcast};

use crate::IFC;

pub trait IfcVerify {
    fn verify_id_types(&self, _ifc: &IFC) -> bool {
        true
    }
}

pub trait IfcType: Downcast + Any + Display + IfcVerify {}
impl_downcast!(IfcType);
