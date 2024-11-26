use std::fmt::Display;

use ifc_rs_verify_derive::IfcVerify;

use crate::{
    id::{Id, IdOr, TypedId},
    parser::{
        comma::Comma, p_space_or_comment_surrounded, string::StringPrimitive, IFCParse, IFCParser,
    },
    prelude::*,
};

/// IfcRelFillsElement is an objectified relationship between an opening
/// element and an element that fills (or partially fills) the opening element.
/// It is an one-to-one relationship.
///
/// https://standards.buildingsmart.org/IFC/DEV/IFC4_2/FINAL/HTML/schema/ifcproductextension/lexical/ifcrelfillselement.htm
#[derive(IfcVerify)]
pub struct RelFillsElement {
    root: Root,
    /// Opening Element being filled by virtue of this relationship.
    relating_opening_element: TypedId<OpeningElement>,
    /// Reference to building element that occupies fully or partially the
    /// associated opening.
    #[ifc_types(Building, OpeningElement, Slab, Wall, Window, Door)]
    pub related_building_element: Id,
}

impl RelFillsElement {
    pub fn new<S: Structure>(
        name: impl Into<StringPrimitive>,
        relating_opening_element: impl Into<IdOr<OpeningElement>>,
        relating_building_element: impl Into<IdOr<S>>,
        ifc: &mut IFC,
    ) -> Self {
        Self {
            root: Root::new(name.into()),
            relating_opening_element: relating_opening_element.into().or_insert(ifc),
            related_building_element: relating_building_element.into().or_insert(ifc).id(),
        }
    }
}

impl RootBuilder for RelFillsElement {
    fn root_mut(&mut self) -> &mut Root {
        &mut self.root
    }
}

impl IFCParse for RelFillsElement {
    fn parse<'a>() -> impl IFCParser<'a, Self> {
        winnow::seq! {
            RelFillsElement {
                _: p_space_or_comment_surrounded("IFCRELFILLSELEMENT("),

                root: Root::parse(),
                _ :Comma::parse(),
                relating_opening_element: Id::parse().map(TypedId::new),
                _: Comma::parse(),
                related_building_element: Id::parse(),

                _: p_space_or_comment_surrounded(");"),
            }
        }
    }
}

impl Display for RelFillsElement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "IFCRELFILLSELEMENT({},{},{});",
            self.root, self.relating_opening_element, self.related_building_element
        )
    }
}

impl IfcType for RelFillsElement {}

#[cfg(test)]
mod test {
    use winnow::Parser;

    use super::RelFillsElement;
    use crate::parser::IFCParse;

    #[test]
    fn rel_fills_element_round_trip() {
        let example = "IFCRELFILLSELEMENT('0YVioT$0bDzPFxfmI$Sb2G',#2,$,$,#44,#47);";

        let parsed: RelFillsElement = RelFillsElement::parse().parse(example).unwrap();
        let str = parsed.to_string();

        assert_eq!(example, str);
    }
}
