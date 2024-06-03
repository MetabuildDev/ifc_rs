use std::{fmt::Display, ops::Deref};

use crate::parser::{
    label::Label,
    optional::{IFCParse, OptionalParameter},
    p_space_or_comment_surrounded, IFCParser,
};

use super::type_product::TypeProduct;

pub struct ElementType {
    type_product: TypeProduct,

    pub element_type: OptionalParameter<Label>,
}

impl Deref for ElementType {
    type Target = TypeProduct;

    fn deref(&self) -> &Self::Target {
        &self.type_product
    }
}

impl IFCParse for ElementType {
    fn parse<'a>() -> impl IFCParser<'a, Self> {
        winnow::seq! {
            Self {
                type_product: TypeProduct::parse(),
                _: p_space_or_comment_surrounded(","),
                element_type: OptionalParameter::parse(),
            }
        }
    }
}

impl Display for ElementType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{},{}", self.type_product, self.element_type,)
    }
}
