mod deserialize;
mod serialize;

use std::ops::{Deref, DerefMut};

use ifc_rs_verify_derive::IfcVerify;

use super::{
    shared::{
        element::{Element, ElementBuilder},
        object::{Object, ObjectBuilder},
        product::{Product, ProductBuilder},
        root::{Root, RootBuilder},
    },
    Structure, StructureType,
};
use crate::{
    id::TypedId,
    parser::{ifc_float::IfcFloat, label::Label, optional::OptionalParameter},
    prelude::*,
};

/// The door is a building element that is predominately used to provide
/// controlled access for people and goods. It includes constructions with
/// hinged, pivoted, sliding, and additionally revolving and folding operations.
/// A door consists of a lining and one or several panels.
///
/// The IfcDoor defines a particular occurrence of a door inserted in the spatial
/// context of a project. A door can:
///
/// * be inserted as a filler in an opening using the IfcRelFillsElement
/// relationship, then the IfcDoor has an inverse attribute FillsVoids provided;
/// * be part of an element assembly, in general an IfcCurtainWall, using the
/// IfcRelAggregates relationship, then the IfcDoor has an inverse attribute
/// Decomposes is provided;
/// * be a "free standing" door, then the IfcDoor has no inverse attributes
/// FillsVoids or Decomposes provided.
///
/// There are two main representations for door occurrences:
///
/// * IfcDoor with a shape representation having
/// RepresentationIdenfifier='Profile' is used for all occurrences of doors,
/// that have a 'Profile' shape representation defined to which a set of shape
/// parameters for lining and framing properties apply. Additionally it requires
/// the provision of an IfcDoorType that references one IfcDoorLiningProperties
/// and one to many IfcDoorPanelProperties;
///
/// * IfcDoor with other shape representations an no assiged
/// IfcDoorLiningProperties and IfcDoorPanelProperties is used for all other
/// occurrences of doors, particularly for doors having only 'Brep', or
/// 'SurfaceModel' geometry without applying shape parameters.
///
/// The actual parameters of the door and/or its shape are defined by the
/// IfcDoor as the object occurrence definition, or by the IfcDoorType as the
/// object type definition. The following parameters are provided:
///
/// * at the IfcDoor for occurrence specific parameters. The IfcDoor specifies:
///   * the door width and height
///   * the door opening direction (by the y-axis of the ObjectPlacement)
/// * at the IfcDoorType, to which the IfcDoor is related by the inverse
/// relationship IsTypedBy pointing to IfcRelDefinesByType, for type parameters
/// common to all occurrences of the same type.
///    * the operation type (single swing, double swing, revolving, etc.)
///    * the door hinge side (by using two different styles for right and left
///    opening doors)
///    * the construction material type
///    * the particular attributes for the lining by the IfcDoorLiningProperties
///    * the particular attributes for the panels by the IfcDoorPanelProperties
///
/// The geometric representation of IfcDoor is given by the
/// IfcProductDefinitionShape, allowing multiple geometric representations.
/// The IfcDoor may get its parameter and shape from the IfcDoorType. If an
/// IfcRepresentationMap (a block definition) is defined for the IfcDoorType,
/// then the IfcDoor inserts it through the IfcMappedItem.
///
/// The geometric representation of IfcDoor is defined using the following
/// (potentially multiple) IfcShapeRepresentation's for its
/// IfcProductDefinitionShape:
///
/// * Profile: A 'Curve3D' consisting of a single losed curve defining the
/// outer boundary of the door (lining). The door parametric representation
/// uses this profile in order to apply the door lining and panel parameter.
/// If not provided, the profile of the IfcOpeningElement is taken.
/// * FootPrint: A 'GeometricCurveSet', or 'Annotation2D' representation
/// defining the 2D shape of the door
/// * Body: A 'SweptSolid', 'SurfaceModel', or 'Brep' representation defining
/// the 3D shape of the door.
///
/// In addition the parametric representation of a (limited) door shape is
/// available by applying the parameters from IfcDoorType referencing
/// IfcDoorLiningProperties and IfcDoorPanelProperties. The purpose of the
/// parameter is described at those entities and below (door opening operation
/// by door type). The overall size of the IfcDoor to be used to apply the
/// lining or panel parameter provided by the IfcDoorType is determined by the
/// IfcShapeRepresentation with the RepresentationIdentifier = 'Profile'.
///
/// https://standards.buildingsmart.org/IFC/RELEASE/IFC4/ADD2_TC1/HTML/schema/ifcsharedbldgelements/lexical/ifcdoor.htm
#[derive(IfcVerify)]
pub struct Door {
    #[inherited]
    element: Element,

    /// Overall measure of the height, it reflects the Z Dimension of a bounding
    /// box, enclosing the body of the door opening. If omitted, the
    /// OverallHeight should be taken from the geometric representation of the  
    /// IfcOpening in which the door is inserted.
    pub overall_height: OptionalParameter<IfcFloat>,

    /// Overall measure of the width, it reflects the X Dimension of a bounding
    /// box, enclosing the door opening. If omitted, the OverallWidth should
    /// be taken from the geometric representation of the IfcOpening in which
    /// the door is inserted.
    pub overall_width: OptionalParameter<IfcFloat>,

    /// Predefined generic type for a door that is specified in an
    /// enumeration. There may be a property set given specificly for the
    /// predefined types.
    pub predefined_type: OptionalParameter<DoorTypeEnum>,

    /// Type defining the general layout and operation of the door type in
    /// terms of the partitioning of panels and panel operations.
    pub operation_type: OptionalParameter<DoorOperationTypeEnum>,

    /// Designator for the user defined operation type, shall only be
    /// provided, if the value of operation_type is set to USERDEFINED.
    pub user_defining_operation_type: OptionalParameter<Label>,
}

impl Door {
    pub fn new(name: impl Into<Label>) -> Self {
        Self {
            element: Element::new(Product::new(Object::new(Root::new(name.into())))),
            overall_height: OptionalParameter::omitted(),
            overall_width: OptionalParameter::omitted(),
            predefined_type: OptionalParameter::omitted(),
            operation_type: OptionalParameter::omitted(),
            user_defining_operation_type: OptionalParameter::omitted(),
        }
    }

    pub fn overall_height(mut self, overall_height: impl Into<IfcFloat>) -> Self {
        self.overall_height = overall_height.into().into();
        self
    }

    pub fn overall_width(mut self, overall_width: impl Into<IfcFloat>) -> Self {
        self.overall_width = overall_width.into().into();
        self
    }

    pub fn predefined_type(mut self, predefined_type: impl Into<DoorTypeEnum>) -> Self {
        self.predefined_type = predefined_type.into().into();
        self
    }

    pub fn operation_type(mut self, operation_type: impl Into<DoorOperationTypeEnum>) -> Self {
        self.operation_type = operation_type.into().into();
        self
    }
}

impl RootBuilder for Door {
    fn root_mut(&mut self) -> &mut Root {
        &mut self.element
    }
}

impl ObjectBuilder for Door {
    fn object_mut(&mut self) -> &mut Object {
        &mut self.element
    }
}

impl ProductBuilder for Door {
    fn product_mut(&mut self) -> &mut Product {
        &mut self.element
    }
}

impl ElementBuilder for Door {
    fn element_mut(&mut self) -> &mut Element {
        &mut self.element
    }
}

impl Deref for Door {
    type Target = Element;

    fn deref(&self) -> &Self::Target {
        &self.element
    }
}

impl DerefMut for Door {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.element
    }
}

impl IfcType for Door {
    fn to_structure(&self) -> Option<&dyn Structure> {
        Some(self)
    }
}
impl Structure for Door {
    fn structure_type(&self) -> Option<StructureType<'_>> {
        Some(StructureType::Door(self))
    }
}
impl MaterialRelatable for Door {}

impl TransformableType for Door {
    fn shape(&self) -> Option<TypedId<ProductDefinitionShape>> {
        self.representation.custom().cloned()
    }
}
