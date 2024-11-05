use std::ops::Deref;

use door_operation_type_enum::DoorOperationTypeEnum;
use door_type_enum::DoorTypeEnum;
use ifc_rs_verify_derive::IfcVerify;

use crate::{
    parser::{bool::BoolPrimitive, optional::OptionalParameter, string::StringPrimitive},
    prelude::*,
};

use super::shared::{
    element_type::ElementType, type_object::TypeObjectBuilder, type_product::TypeProductBuilder,
};

mod deserialize;
pub(crate) mod door_operation_type_enum;
pub(crate) mod door_type_enum;
mod serialize;

/// The element type IfcDoorType defines commonly shared information for
/// occurrences of doors. The set of shared information may include:
///
/// * common properties within shared property sets
/// * common material information
/// * common operation type definitions
/// * common shape representations
///
/// A door type defines the particular parameter of the lining and one
/// (or several) panels through the IfcDoorLiningProperties and the
/// IfcDoorPanelProperties as predefined property sets applicable to doors only.
///
/// It is used to define a door specification, or door style (i.e. the specific
/// product information that is common to all occurrences of that door type).
/// Door types may be exchanged without being already assigned to occurrences.
///
/// https://standards.buildingsmart.org/IFC/RELEASE/IFC4/ADD2_TC1/HTML/schema/ifcsharedbldgelements/lexical/ifcdoortype.htm
#[derive(IfcVerify)]
pub struct DoorType {
    #[inherited]
    element_type: ElementType,

    /// Identifies the predefined types of a door element from which the
    /// type required may be set.
    pub predefined_type: DoorTypeEnum,

    /// Type defining the general layout and operation of the door type in terms
    /// of the partitioning of panels and panel operations.
    pub operation_type: DoorOperationTypeEnum,

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

impl DoorType {
    pub fn new(
        name: impl Into<StringPrimitive>,
        predefined_type: DoorTypeEnum,
        operation_type: DoorOperationTypeEnum,
    ) -> Self {
        Self {
            element_type: ElementType::new(TypeProduct::new(TypeObject::new(Root::new(
                name.into(),
            )))),
            predefined_type,
            operation_type,
            parameter_takes_precedence: OptionalParameter::omitted(),
            user_defined_partitioning_type: OptionalParameter::omitted(),
        }
    }
}

impl ElementTypeBuilder for DoorType {
    fn element_type_mut(&mut self) -> &mut ElementType {
        &mut self.element_type
    }
}

impl TypeProductBuilder for DoorType {
    fn type_product_mut(&mut self) -> &mut TypeProduct {
        &mut self.element_type
    }
}

impl TypeObjectBuilder for DoorType {
    fn type_object_mut(&mut self) -> &mut TypeObject {
        &mut self.element_type
    }
}

impl RootBuilder for DoorType {
    fn root_mut(&mut self) -> &mut Root {
        &mut self.element_type
    }
}

impl Deref for DoorType {
    type Target = ElementType;

    fn deref(&self) -> &Self::Target {
        &self.element_type
    }
}

impl IfcType for DoorType {}
impl MaterialRelatable for DoorType {}

#[cfg(test)]
mod test {
    use winnow::Parser;

    use crate::parser::IFCParse;

    use super::DoorType;

    #[test]
    fn door_type_round_trip() {
        let example = "IFCDOORTYPE('3rpRbH5aaBek8KKG_Q7ddy',#12,'IFC T20FC0 - Eine 20D60nunsgrichtung',$,$,(#17731,#17734),(#17743,#17745),'F5CDB951-1649-0BA2-E214-510F9A1E79FC',$,.NOTDEFINED.,.SINGLE_SWING_RIGHT.,.TRUE.,$);";

        let door_type = DoorType::parse().parse(example).unwrap();
        let str_door_type = door_type.to_string();

        assert_eq!(example, str_door_type);
    }
}
