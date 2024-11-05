use std::ops::Deref;

use ifc_rs_verify_derive::IfcVerify;
use window_partitioning_type_enum::WindowPartitioningTypeEnum;
use window_type_enum::WindowTypeEnum;

use crate::{
    parser::{bool::BoolPrimitive, optional::OptionalParameter, string::StringPrimitive},
    prelude::*,
};

use super::shared::{
    element_type::ElementType, type_object::TypeObjectBuilder, type_product::TypeProductBuilder,
};

mod deserialize;
mod serialize;
pub(crate) mod window_partitioning_type_enum;
pub(crate) mod window_type_enum;

/// The element type IfcWindowType defines commonly shared information for
/// occurrences of windows. The set of shared information may include:
///
/// * common properties within shared property sets
/// * common material information
/// * common partitioning of panels
/// * common operation types of panels
/// * common shape representations
///
/// A window type defines the particular parameter of the lining and one
/// (or several) panels through the IfcWindowLiningProperties and the
/// IfcWindowPanelProperties as predefined property sets applicable to windows
/// only.
///
/// It is used to define a window specification, or window style (the specific
/// product information that is common to all occurrences of that window type).
/// Window types may be exchanged without being already assigned to occurrences.
///
/// Occurrences of the IfcWindowType within building models are represented by
/// instances of IfcWindow or IfcWindowStandardCase.
///
/// https://standards.buildingsmart.org/IFC/DEV/IFC4_2/FINAL/HTML/schema/ifcsharedbldgelements/lexical/ifcwindowtype.htm
#[derive(IfcVerify)]
pub struct WindowType {
    #[inherited]
    element_type: ElementType,

    /// Identifies the predefined types of a window element from which the
    /// type required may be set.
    pub predefined_type: WindowTypeEnum,

    /// Type defining the general layout of the window type in terms of the
    /// partitioning of panels.
    pub partitioning_type: WindowPartitioningTypeEnum,

    /// The Boolean value reflects, whether the parameter given in the attached
    /// lining and panel properties exactly define the geometry (TRUE), or
    /// whether the attached style shape take precedence (FALSE). In the last
    /// case the parameter have only informative value. If not provided, no
    /// such information can be infered.
    pub parameter_takes_precedence: OptionalParameter<BoolPrimitive>,

    /// Designator for the user defined partitioning type, shall only be
    /// provided, if the value of PartitioningType is set to USERDEFINED.
    pub user_defined_partitioning_type: OptionalParameter<StringPrimitive>,
}

impl WindowType {
    pub fn new(
        name: impl Into<StringPrimitive>,
        predefined_type: WindowTypeEnum,
        partitioning_type: WindowPartitioningTypeEnum,
    ) -> Self {
        Self {
            element_type: ElementType::new(TypeProduct::new(TypeObject::new(Root::new(
                name.into(),
            )))),
            predefined_type,
            partitioning_type,
            parameter_takes_precedence: OptionalParameter::omitted(),
            user_defined_partitioning_type: OptionalParameter::omitted(),
        }
    }
}

impl ElementTypeBuilder for WindowType {
    fn element_type_mut(&mut self) -> &mut ElementType {
        &mut self.element_type
    }
}

impl TypeProductBuilder for WindowType {
    fn type_product_mut(&mut self) -> &mut TypeProduct {
        &mut self.element_type
    }
}

impl TypeObjectBuilder for WindowType {
    fn type_object_mut(&mut self) -> &mut TypeObject {
        &mut self.element_type
    }
}

impl RootBuilder for WindowType {
    fn root_mut(&mut self) -> &mut Root {
        &mut self.element_type
    }
}

impl Deref for WindowType {
    type Target = ElementType;

    fn deref(&self) -> &Self::Target {
        &self.element_type
    }
}

impl IfcType for WindowType {}
impl MaterialRelatable for WindowType {}

#[cfg(test)]
mod test {
    use winnow::Parser;

    use crate::parser::IFCParse;

    use super::WindowType;

    #[test]
    fn window_type_round_trip() {
        let example = "IFCWINDOWTYPE('0Ps4H3X0nAxfqkHNemLE6f',#2,'Window for Test Example','Description of Window Type',$,$,$,$,$,.WINDOW.,.SINGLE_PANEL.,$,$);";

        let wall_type: WindowType = WindowType::parse().parse(example).unwrap();
        let str_wall_type = wall_type.to_string();

        assert_eq!(example, str_wall_type);
    }
}
