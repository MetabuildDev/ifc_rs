use std::{fmt::Display, ops::Deref};

use crate::{
    id::Id,
    parser::{
        optional::{IFCParse, OptionalParameter},
        p_space_or_comment_surrounded, IFCParser,
    },
};

use super::product::Product;

/// An element is a generalization of all components that make up an AEC product.
///
/// https://standards.buildingsmart.org/IFC/DEV/IFC4_2/FINAL/HTML/schema/ifcproductextension/lexical/ifcelement.htm
pub struct Element {
    product: Product,

    /// The tag (or label) identifier at the particular instance of a product,
    /// e.g. the serial number, or the position number. It is the identifier
    /// at the occurrence level.
    pub tag: OptionalParameter<Id>,
}

impl Deref for Element {
    type Target = Product;

    fn deref(&self) -> &Self::Target {
        &self.product
    }
}

impl IFCParse for Element {
    fn parse<'a>() -> impl IFCParser<'a, Self> {
        winnow::seq! {
            Self {
                product: Product::parse(),
                _: p_space_or_comment_surrounded(","),
                tag: OptionalParameter::parse(),
            }
        }
    }
}

impl Display for Element {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{},{}", self.product, self.tag)
    }
}
