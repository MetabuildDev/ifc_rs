use crate::prelude::*;

pub trait IfcMappedType<'a> {
    type Target;

    fn mappings(&'a self, ifc: &'a IFC) -> Self::Target;
}
