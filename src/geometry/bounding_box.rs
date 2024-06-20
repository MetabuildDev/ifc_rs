use std::fmt::Display;

use ifc_verify_derive::IfcVerify;

use crate::{
    id::Id,
    ifc_type::{IfcType, IfcVerify},
    parser::{
        comma::Comma, ifc_float::IfcFloat, p_space_or_comment_surrounded, IFCParse, IFCParser,
    },
    IFC,
};

/// Dummy implementation for now
///
/// https://standards.buildingsmart.org/IFC/DEV/IFC4_2/FINAL/HTML/link/ifcboundingbox.htm
#[derive(IfcVerify)]
pub struct BoundingBox {}

impl IFCParse for BoundingBox {
    fn parse<'a>() -> impl IFCParser<'a, Self> {
        winnow::seq! {
            Self {
                _: p_space_or_comment_surrounded("IFCBOUNDINGBOX("),

                _: Id::parse(),
                _: Comma::parse(),
                _: IfcFloat::parse(),
                _: Comma::parse(),
                _: IfcFloat::parse(),
                _: Comma::parse(),
                _: IfcFloat::parse(),

                _: p_space_or_comment_surrounded(");"),
            }
        }
    }
}

impl Display for BoundingBox {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "IFCBOUNDINGBOX($,$,$,$);",)
    }
}

impl IfcType for BoundingBox {}
