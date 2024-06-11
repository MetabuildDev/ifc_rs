use std::{fmt::Display, ops::Deref};

use crate::{
    id::Id,
    ifc_type::IfcType,
    parser::{
        comma::Comma, list::IfcList, optional::OptionalParameter, p_space_or_comment_surrounded,
        IFCParse, IFCParser,
    },
};

use super::shared::root::Root;

/// The aggregation relationship IfcRelAggregates is a special type of
/// the general composition/decomposition (or whole/part) relationship
/// IfcRelDecomposes. The aggregation relationship can be applied to
/// all subtypes of IfcObjectDefinition.
///
/// https://standards.buildingsmart.org/IFC/DEV/IFC4_2/FINAL/HTML/link/ifcrelaggregates.htm
pub struct RelAggregates {
    root: Root,

    /// The object definition, either an object type or an object
    /// occurrence, that represents the aggregation. It is the whole
    /// within the whole/part relationship.
    pub relating_object: OptionalParameter<Id>,

    /// The object definitions, either object occurrences or object
    /// types, that are being aggregated. They are defined as the
    /// parts in the whole/part relationship. No order is implied
    /// between the parts.
    pub related_objects: IfcList<Id>,
}

impl RelAggregates {
    // pub fn new(
    //     global_id: Label,
    //     owner_history: OptionalParameter<Id>,
    //     name: OptionalParameter<Label>,
    //     description: OptionalParameter<Label>,
    //     relating_object: OptionalParameter<Id>,
    //     related_objects: IfcList<Id>,
    // ) -> Self {
    //     Self {
    //         root,
    //         relating_object,
    //         related_objects,
    //     }
    // }

    // pub fn
}

impl Deref for RelAggregates {
    type Target = Root;

    fn deref(&self) -> &Self::Target {
        &self.root
    }
}

impl IFCParse for RelAggregates {
    fn parse<'a>() -> impl IFCParser<'a, Self> {
        winnow::seq! {
            Self {
                _: p_space_or_comment_surrounded("IFCRELAGGREGATES("),

                root: Root::parse(),
                _: Comma::parse(),
                relating_object: OptionalParameter::parse(),
                _: Comma::parse(),
                related_objects: IfcList::parse(),

                _: p_space_or_comment_surrounded(");"),
            }
        }
    }
}

impl Display for RelAggregates {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "IFCRELAGGREGATES({},{},{});",
            self.root, self.relating_object, self.related_objects
        )
    }
}

impl IfcType for RelAggregates {}

#[cfg(test)]
mod test {
    use winnow::Parser;

    use super::RelAggregates;
    use crate::parser::IFCParse;

    #[test]
    fn rel_aggregates_round_trip() {
        let example = "IFCRELAGGREGATES('091a6ewbvCMQ2Vyiqspa7a',#2,'Project Container','Project Container for Buildings',#10,(#1));";

        let rel_aggregates: RelAggregates = RelAggregates::parse().parse(example).unwrap();
        let str_rel_aggregates = rel_aggregates.to_string();

        assert_eq!(example, str_rel_aggregates);
    }
}
