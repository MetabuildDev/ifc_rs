use std::{fmt::Display, ops::Deref};

use crate::{
    id::Id,
    parser::{
        label::Label,
        optional::{IFCParse, OptionalParameter},
        p_space_or_comment_surrounded, IFCParser,
    },
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
pub struct TypeProduct {
    type_object: TypeObject,

    /// List of unique representation maps. Each representation map
    /// describes a block definition of the shape of the product style.
    /// By providing more than one representation map, a multi-view
    /// block definition can be given.
    pub representation_maps: OptionalParameter<Id>,

    /// The tag (or label) identifier at the particular type of a
    /// product, e.g. the article number (like the EAN). It is the
    /// identifier at the specific level.
    pub tag: OptionalParameter<Label>,
}

impl Deref for TypeProduct {
    type Target = TypeObject;

    fn deref(&self) -> &Self::Target {
        &self.type_object
    }
}

impl IFCParse for TypeProduct {
    fn parse<'a>() -> impl IFCParser<'a, Self> {
        winnow::seq! {
            Self {
                type_object: TypeObject::parse(),
                _: p_space_or_comment_surrounded(","),
                representation_maps: OptionalParameter::parse(),
                _: p_space_or_comment_surrounded(","),
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
