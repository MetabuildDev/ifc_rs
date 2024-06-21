mod deserialize;
mod serialize;

use std::ops::{Deref, DerefMut};

use ifc_verify_derive::IfcVerify;

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
    ifc_type::{IfcType, IfcVerify},
    objects::windowtype::{
        window_partitioning_type_enum::WindowPartitioningTypeEnum, window_type_enum::WindowTypeEnum,
    },
    parser::{ifc_float::IfcFloat, label::Label, optional::OptionalParameter},
    prelude::{ProductDefinitionShape, TransformableType},
    relations::rel_associates_material::MaterialRelatable,
    IFC,
};

/// The window is a building element that is predominately used to provide
/// natural light and fresh air. It includes vertical opening but also
/// horizontal opening such as skylights or light domes. It includes
/// constructions with swinging, pivoting, sliding, or revolving panels and
/// fixed panels. A window consists of a lining and one or several panels.
///
/// The IfcWindow defines a particular occurrence of a window inserted in the
/// spatial context of a project. A window can:
///
/// * be inserted into an IfcOpeningElement using the IfcRelFillsElement
/// relationship, then the IfcWindow has an inverse attribute FillsVoids provided,
/// * be part of an element assembly, often an IfcCurtainWall, using the
/// IfcRelAggregates relationship, then the inverse attribute Decomposes is provided.
/// * or be a "free standing" window, then the IfcWindow has no inverse
/// attributes FillsVoids or Decomposes provided.
///
/// Note: View definitions or implementer agreements may restrict the
/// relationship to only include one window (or door) into one opening.
///
/// There are two entities for window occurrences:
///
/// * IfcWindowStandardCase used for all occurrences of windows, that have a
/// 'Profile' shape representation defined to which a set of shape parameters
/// for lining and framing properties apply. Additionally it requires the
/// provision of an IfcWindowType that references one IfcWindowLiningProperties
/// and on to many IfcWindowPanelProperties.
/// * IfcWindow used for all other occurrences of windows, particularly for
/// windows having only 'Brep', or 'SurfaceModel' geometry without applying
/// shape parameters.
///
/// The actual parameter of the window and/or its shape is defined at the
/// IfcWindow as the occurrence definition (or project instance), or by the
/// IfcWindowType as the specific definition (or project type). The following
/// parameters are given:
///
/// * at the IfcWindow or IfcWindowStandardCase for occurrence specific
/// parameters. The IfcWindow specifies:
///   * the window width and height
///   * the window opening direction (by the y-axis of the ObjectPlacement)
///
/// * at the IfcWindowType to which the IfcWindow is related by the inverse
/// relationship IsDefinedBy pointing to IfcRelDefinesByType, for type
/// parameters common to all occurrences of the same type.
///   * the partitioning type (single panel, double panel, tripel panel, more panels)
///   * the operation type (swing, tilt and turn, pivot revolve, fixed case ment, etc.)
///   * the window panel hinge side (by using two different styles for right and left opening windows)
///   * the construction material type
///   * the particular attributes for the lining by the IfcWindowLiningProperties
///   * the particular attributes for the panels by the  IfcWindowPanelProperties
///
/// https://standards.buildingsmart.org/IFC/DEV/IFC4_2/FINAL/HTML/schema/ifcsharedbldgelements/lexical/ifcwindow.htm
#[derive(IfcVerify)]
pub struct Window {
    #[inherited]
    element: Element,

    /// Overall measure of the height, it reflects the Z Dimension of a bounding
    /// box, enclosing the window opening. If omitted, the OverallHeight should
    /// be taken from the geometric representation of the IfcOpening in which
    /// the window is inserted.
    pub overall_height: OptionalParameter<IfcFloat>,

    /// Overall measure of the width, it reflects the X Dimension of a bounding
    /// box, enclosing the window opening. If omitted, the OverallWidth should
    /// be taken from the geometric representation of the IfcOpening in which
    /// the window is inserted.
    pub overall_width: OptionalParameter<IfcFloat>,

    /// Predefined generic type for an window that is specified in an
    /// enumeration. There may be a property set given specificly for the
    /// predefined types.
    pub predefined_type: OptionalParameter<WindowTypeEnum>,

    /// Type defining the general layout of the window in terms of the
    /// partitioning of panels.
    pub partitioning_type: OptionalParameter<WindowPartitioningTypeEnum>,

    /// Designator for the user defined partitioning type, shall only be
    /// provided, if the value of PartitioningType is set to USERDEFINED.
    pub user_defining_partitioning_type: OptionalParameter<Label>,
}

impl Window {
    pub fn new<'a>(name: impl Into<Label>) -> Self {
        Self {
            element: Element::new(Product::new(Object::new(Root::new(name.into())))),
            overall_height: OptionalParameter::omitted(),
            overall_width: OptionalParameter::omitted(),
            predefined_type: OptionalParameter::omitted(),
            partitioning_type: OptionalParameter::omitted(),
            user_defining_partitioning_type: OptionalParameter::omitted(),
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

    pub fn predefined_type(mut self, predefined_type: impl Into<WindowTypeEnum>) -> Self {
        self.predefined_type = predefined_type.into().into();
        self
    }

    pub fn partitioning_type(
        mut self,
        partitioning_type: impl Into<WindowPartitioningTypeEnum>,
    ) -> Self {
        self.partitioning_type = partitioning_type.into().into();
        self
    }
}

impl RootBuilder for Window {
    fn root_mut(&mut self) -> &mut Root {
        &mut self.element
    }
}

impl ObjectBuilder for Window {
    fn object_mut(&mut self) -> &mut Object {
        &mut self.element
    }
}

impl ProductBuilder for Window {
    fn product_mut(&mut self) -> &mut Product {
        &mut self.element
    }
}

impl ElementBuilder for Window {
    fn element_mut(&mut self) -> &mut Element {
        &mut self.element
    }
}

impl Deref for Window {
    type Target = Element;

    fn deref(&self) -> &Self::Target {
        &self.element
    }
}

impl DerefMut for Window {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.element
    }
}

impl IfcType for Window {
    fn to_structure(&self) -> Option<&dyn Structure> {
        Some(self)
    }
}
impl Structure for Window {
    fn structure_type(&self) -> Option<StructureType<'_>> {
        Some(StructureType::Window(self))
    }
}
impl MaterialRelatable for Window {}

impl TransformableType for Window {
    fn shape(&self) -> Option<TypedId<ProductDefinitionShape>> {
        self.representation.custom().cloned()
    }
}
