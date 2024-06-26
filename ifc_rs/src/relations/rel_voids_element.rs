use std::fmt::Display;

use ifc_rs_verify_derive::IfcVerify;

use crate::{
    id::{Id, IdOr, TypedId},
    parser::{comma::Comma, label::Label, p_space_or_comment_surrounded, IFCParse, IFCParser},
    prelude::*,
};

/// IfcRelVoidsElement is an objectified relationship between a building
/// element and one opening element that creates a void in the element.
/// It is a one-to-one relationship. This relationship implies a Boolean
/// operation of subtraction between the geometric bodies of the element and
/// the opening.
///
/// https://standards.buildingsmart.org/IFC/DEV/IFC4_2/FINAL/HTML/schema/ifcproductextension/lexical/ifcrelvoidselement.htm

#[derive(IfcVerify)]
pub struct RelVoidsElement {
    root: Root,
    /// Reference to element in which a void is created by associated feature
    /// subtraction element.
    #[ifc_types(Building, OpeningElement, Slab, Wall, Window)]
    pub relating_building_element: Id,
    /// Reference to the feature subtraction element which defines a void in
    /// the associated element.
    pub related_opening_element: TypedId<OpeningElement>,
}

impl RelVoidsElement {
    pub fn new<S: Structure>(
        name: impl Into<Label>,
        relating_building_element: impl Into<IdOr<S>>,
        relating_opening_element: impl Into<IdOr<OpeningElement>>,
        ifc: &mut IFC,
    ) -> Self {
        Self {
            root: Root::new(name.into()),
            relating_building_element: relating_building_element.into().or_insert(ifc).id(),
            related_opening_element: relating_opening_element.into().or_insert(ifc),
        }
    }
}

impl RootBuilder for RelVoidsElement {
    fn root_mut(&mut self) -> &mut Root {
        &mut self.root
    }
}

impl IFCParse for RelVoidsElement {
    fn parse<'a>() -> impl IFCParser<'a, Self> {
        winnow::seq! {
            Self {
                _: p_space_or_comment_surrounded("IFCRELVOIDSELEMENT("),

                root: Root::parse(),
                _ :Comma::parse(),
                relating_building_element: Id::parse(),
                _: Comma::parse(),
                related_opening_element: Id::parse().map(TypedId::new),

                _: p_space_or_comment_surrounded(");"),
            }
        }
    }
}

impl Display for RelVoidsElement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "IFCRELVOIDSELEMENT({},{},{});",
            self.root, self.relating_building_element, self.related_opening_element
        )
    }
}

impl IfcType for RelVoidsElement {}

#[cfg(test)]
mod test {
    use winnow::Parser;

    use super::RelVoidsElement;
    use crate::parser::IFCParse;

    #[test]
    fn rel_voids_element_round_trip() {
        let example = "IFCRELVOIDSELEMENT('1nwVYC$VTDeuSc8zbOa89u',#2,$,$,#38,#44);";

        let parsed: RelVoidsElement = RelVoidsElement::parse().parse(example).unwrap();
        let str = parsed.to_string();

        assert_eq!(example, str);
    }
}
