use comma::Comma;

use crate::{
    objects::{
        shared::element_type::ElementType,
        windowtype::{
            deserialize::optional::OptionalParameter,
            window_partitioning_type_enum::WindowPartitioningTypeEnum,
            window_type_enum::WindowTypeEnum,
        },
    },
    parser::*,
};

use super::WindowType;

impl IFCParse for WindowType {
    fn parse<'a>() -> impl IFCParser<'a, Self> {
        winnow::seq! {
            WindowType {
                _: p_space_or_comment_surrounded("IFCWINDOWTYPE("),

                element_type: ElementType::parse(),
                _: Comma::parse(),
                predefined_type: WindowTypeEnum::parse(),
                _: Comma::parse(),
                partitioning_type: WindowPartitioningTypeEnum::parse(),
                _: Comma::parse(),
                parameter_takes_precedence: OptionalParameter::parse(),
                _: Comma::parse(),
                user_defined_partitioning_type: OptionalParameter::parse(),

                _: p_space_or_comment_surrounded(");"),
            }
        }
    }
}
