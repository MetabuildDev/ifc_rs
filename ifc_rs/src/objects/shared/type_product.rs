use std::{
    fmt::Display,
    ops::{Deref, DerefMut},
};

use ifc_rs_verify_derive::IfcVerify;

use crate::{
    id::TypedId,
    parser::{
        comma::Comma, list::IfcList, optional::OptionalParameter, string::StringPrimitive,
        IFCParse, IFCParser,
    },
    prelude::*,
};

use super::type_object::TypeObject;

/// IfcTypeProduct defines a type definition of a product without being
/// already inserted into a project structure (without having a placement),
/// and not being included in the geometric representation context of
/// the project. It is used to define a product specification, that is,
/// the specific product information that is common to all occurrences
/// of that product type.
///
/// https://standards.buildingsmart.org/IFC/DEV/IFC4_2/FINAL/HTML/schema/ifckernel/lexical/ifctypeproduct.htm
#[derive(IfcVerify)]
pub struct TypeProduct {
    #[inherited]
    type_object: TypeObject,

    /// List of unique representation maps. Each representation map
    /// describes a block definition of the shape of the product style.
    /// By providing more than one representation map, a multi-view
    /// block definition can be given.
    pub representation_maps: OptionalParameter<IfcList<TypedId<RepresentationMap>>>,

    /// The tag (or label) identifier at the particular type of a
    /// product, e.g. the article number (like the EAN). It is the
    /// identifier at the specific level.
    pub tag: OptionalParameter<StringPrimitive>,
}

impl TypeProduct {
    pub fn new(type_object: TypeObject) -> Self {
        Self {
            type_object,
            representation_maps: OptionalParameter::omitted(),
            tag: OptionalParameter::omitted(),
        }
    }
}

pub trait TypeProductBuilder: Sized {
    fn type_product_mut(&mut self) -> &mut TypeProduct;

    // TODO
    // fn representation_maps(mut self, representation_maps: impl Into<IdOr< /* TODO */ >>, ifc: &mut IFC) -> Self {
    //     self.type_product_mut().representation_maps = representation_maps.into().or_insert(ifc).id().into();
    //     self
    // }

    fn tag(mut self, tag: impl Into<StringPrimitive>) -> Self {
        self.type_product_mut().tag = tag.into().into();
        self
    }
}

impl Deref for TypeProduct {
    type Target = TypeObject;

    fn deref(&self) -> &Self::Target {
        &self.type_object
    }
}

impl DerefMut for TypeProduct {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.type_object
    }
}

impl IFCParse for TypeProduct {
    fn parse<'a>() -> impl IFCParser<'a, Self> {
        winnow::seq! {
            TypeProduct {
                type_object: TypeObject::parse(),
                _: Comma::parse(),
                representation_maps: OptionalParameter::parse(),
                _: Comma::parse(),
                tag: OptionalParameter::parse()
            }
        }
    }
}

impl Display for TypeProduct {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{},{},{}",
            self.type_object, self.representation_maps, self.tag
        )
    }
}
