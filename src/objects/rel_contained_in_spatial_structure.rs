use std::{fmt::Display, ops::Deref};

use crate::id::Id;
use crate::ifc_type::IfcType;
use crate::parser::comma::Comma;
use crate::parser::list::IfcList;
use crate::parser::p_space_or_comment_surrounded;
use crate::parser::IFCParse;
use crate::parser::IFCParser;

use super::shared::root::Root;

/// This objectified relationship, IfcRelContainedInSpatialStructure,
/// is used to assign elements to a certain level of the spatial project
/// structure. Any element can only be assigned once to a certain level
/// of the spatial structure. The question, which level is relevant
/// for which type of element, can only be answered within the context
/// of a particular project and might vary within the various regions.
///
/// https://standards.buildingsmart.org/IFC/DEV/IFC4_2/FINAL/HTML/link/ifcrelcontainedinspatialstructure.htm
pub struct RelContainedInSpatialStructure {
    root: Root,

    /// Set of products, which are contained within this level of the
    /// spatial structure hierarchy.
    pub related_elements: IfcList<Id>,

    /// Spatial structure element, within which the element is
    /// contained. Any element can only be contained within one
    /// element of the project spatial structure.
    pub relating_structure: Id,
}

impl Deref for RelContainedInSpatialStructure {
    type Target = Root;

    fn deref(&self) -> &Self::Target {
        &self.root
    }
}

impl IFCParse for RelContainedInSpatialStructure {
    fn parse<'a>() -> impl IFCParser<'a, Self> {
        winnow::seq! {
            Self {
                _: p_space_or_comment_surrounded("IFCRELCONTAINEDINSPATIALSTRUCTURE("),

                root: Root::parse(),
                _: Comma::parse(),
                related_elements: IfcList::parse(),
                _: Comma::parse(),
                relating_structure: Id::parse(),

                _: p_space_or_comment_surrounded(");"),
            }
        }
    }
}

impl Display for RelContainedInSpatialStructure {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "IFCRELCONTAINEDINSPATIALSTRUCTURE({},{},{});",
            self.root, self.related_elements, self.relating_structure
        )
    }
}

impl IfcType for RelContainedInSpatialStructure {}

#[cfg(test)]
mod test {
    use winnow::Parser;

    use super::RelContainedInSpatialStructure;
    use crate::parser::IFCParse;

    #[test]
    fn rel_contained_in_spatial_structure_round_trip() {
        let example = "IFCRELCONTAINEDINSPATIALSTRUCTURE('3Sa3dTJGn0H8TQIGiuGQd5',#2,'Building','Building Container for Elements',(#11),#1);";

        let parsed: RelContainedInSpatialStructure = RelContainedInSpatialStructure::parse()
            .parse(example)
            .unwrap();
        let str = parsed.to_string();

        assert_eq!(example, str);
    }
}
