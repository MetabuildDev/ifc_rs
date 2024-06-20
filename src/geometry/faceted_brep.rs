use std::fmt::Display;

use ifc_verify_derive::IfcVerify;

use crate::{
    id::Id,
    ifc_type::{IfcType, IfcVerify},
    parser::{p_space_or_comment_surrounded, IFCParse, IFCParser},
    IFC,
};

/// Dummy implementation for now
///
/// https://standards.buildingsmart.org/IFC/DEV/IFC4_2/FINAL/HTML/link/ifcfacetedbrep.htm
#[derive(IfcVerify)]
pub struct FacetedBrep {}

impl IFCParse for FacetedBrep {
    fn parse<'a>() -> impl IFCParser<'a, Self> {
        winnow::seq! {
            Self {
                _: p_space_or_comment_surrounded("IFCFACETEDBREP("),

                _: Id::parse(),

                _: p_space_or_comment_surrounded(");"),
            }
        }
    }
}

impl Display for FacetedBrep {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "IFCFACETEDBREP($);",)
    }
}

impl IfcType for FacetedBrep {}
