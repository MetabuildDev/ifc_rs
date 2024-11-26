use comma::Comma;

use crate::{
    objects::{
        doortype::{
            deserialize::optional::OptionalParameter,
            door_operation_type_enum::DoorOperationTypeEnum, door_type_enum::DoorTypeEnum,
        },
        shared::element_type::ElementType,
    },
    parser::*,
};

use super::DoorType;

impl IFCParse for DoorType {
    fn parse<'a>() -> impl IFCParser<'a, Self> {
        winnow::seq! {
            DoorType {
                _: p_space_or_comment_surrounded("IFCDOORTYPE("),

                element_type: ElementType::parse(),
                _: Comma::parse(),
                predefined_type: DoorTypeEnum::parse(),
                _: Comma::parse(),
                operation_type: DoorOperationTypeEnum::parse(),
                _: Comma::parse(),
                parameter_takes_precedence: OptionalParameter::parse(),
                _: Comma::parse(),
                user_defined_partitioning_type: OptionalParameter::parse(),

                _: p_space_or_comment_surrounded(");"),
            }
        }
    }
}
