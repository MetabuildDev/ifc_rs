use std::{fmt::Display, ops::Deref};

use crate::parser::IFCParse;
use crate::parser::{
    label::Label, optional::OptionalParameter, p_space_or_comment_surrounded, IFCParser,
};

use super::type_product::TypeProduct;

/// IfcElementType defines a list of commonly shared property set
/// definitions of an element and an optional set of product
/// representations. It is used to define an element specification
/// (i.e. the specific product information, that is common to
/// all occurrences of that product type).
///
/// https://standards.buildingsmart.org/IFC/DEV/IFC4_2/FINAL/HTML/schema/ifcproductextension/lexical/ifcelementtype.htm
pub struct ElementType {
    type_product: TypeProduct,

    /// The type denotes a particular type that indicates the
    /// object further. The use has to be established at the level
    /// of instantiable subtypes. In particular it holds the user
    /// defined type, if the enumeration of the attribute
    /// 'PredefinedType' is set to USERDEFINED.
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
