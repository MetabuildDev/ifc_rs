use crate::prelude::*;

pub trait Transform3D: IfcType {}

pub trait AxisPlacement: IfcType {}

pub trait Curve: IfcType {}

pub trait CartesianPoint: IfcType {}

pub trait PointList: IfcType {}

pub trait ProfileDef: IfcType {}

pub trait ShapeItem: IfcType {}

pub trait Structure: IfcType {
    fn structure_type(&self) -> Option<StructureType<'_>> {
        None
    }
}

pub trait Address: IfcType {}

pub trait TransformableType: IfcType {
    fn shape(&self) -> Option<TypedId<ProductDefinitionShape>>;
}

/// Material set usages & material sets which can be related to.
pub trait RelatableMaterial: IfcType {}

/// Objects which can be related to materials
pub trait MaterialRelatable: IfcType {}
