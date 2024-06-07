use std::{any::Any, fmt::Display};

use downcast_rs::{self, impl_downcast, Downcast};

pub trait IfcType: Downcast + Any + Display {}
impl_downcast!(IfcType);
