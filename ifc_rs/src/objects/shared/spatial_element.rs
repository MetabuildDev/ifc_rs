use std::{
    fmt::Display,
    ops::{Deref, DerefMut},
};

use ifc_rs_verify_derive::IfcVerify;

use crate::{
    parser::{
        comma::Comma, optional::OptionalParameter, string::StringPrimitive, IFCParse, IFCParser,
    },
    prelude::*,
};

use super::product::Product;

/// A spatial element is the generalization of all spatial elements that
/// might be used to define a spatial structure or to define spatial zones.
///
/// https://standards.buildingsmart.org/IFC/DEV/IFC4_2/FINAL/HTML/schema/ifcproductextension/lexical/ifcspatialelement.htm
#[derive(IfcVerify)]
pub struct SpatialElement {
    #[inherited]
    product: Product,

    /// Long name for a spatial structure element, used for informal purposes.
    /// It should be used, if available, in conjunction with the inherited
    /// Name attribute.
    pub long_name: OptionalParameter<StringPrimitive>,
}

impl SpatialElement {
    pub fn new(product: Product) -> Self {
        Self {
            product,
            long_name: OptionalParameter::omitted(),
        }
    }
}

pub trait SpatialElementBuilder: Sized {
    fn spatial_element_mut(&mut self) -> &mut SpatialElement;

    fn long_name(mut self, long_name: impl Into<StringPrimitive>) -> Self {
        self.spatial_element_mut().long_name = long_name.into().into();
        self
    }
}

impl Deref for SpatialElement {
    type Target = Product;

    fn deref(&self) -> &Self::Target {
        &self.product
    }
}

impl DerefMut for SpatialElement {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.product
    }
}

impl IFCParse for SpatialElement {
    fn parse<'a>() -> impl IFCParser<'a, Self> {
        winnow::seq! {
            SpatialElement {
                product: Product::parse(),
                _: Comma::parse(),
                long_name: OptionalParameter::parse(),
            }
        }
    }
}

impl Display for SpatialElement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{},{}", self.product, self.long_name,)
    }
}
