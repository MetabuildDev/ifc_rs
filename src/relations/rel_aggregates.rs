use std::{fmt::Display, ops::Deref};

use ifc_type_derive::IfcVerify;

use crate::{
    id::{Id, IdOr},
    ifc_type::{IfcType, IfcVerify},
    parser::{
        comma::Comma, label::Label, list::IfcList, optional::OptionalParameter,
        p_space_or_comment_surrounded, IFCParse, IFCParser,
    },
    prelude::{Building, Project, Root, RootBuilder},
    IFC,
};

/// The aggregation relationship IfcRelAggregates is a special type of
/// the general composition/decomposition (or whole/part) relationship
/// IfcRelDecomposes. The aggregation relationship can be applied to
/// all subtypes of IfcObjectDefinition.
///
/// https://standards.buildingsmart.org/IFC/DEV/IFC4_2/FINAL/HTML/link/ifcrelaggregates.htm
#[derive(IfcVerify)]
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
    pub fn new<'a>(global_id: impl Into<Label>) -> Self {
        Self {
            root: Root::new(global_id.into()),
            relating_object: OptionalParameter::omitted(),
            related_objects: IfcList::empty(),
        }
    }

    pub fn relate_project_with_buildings(
        mut self,
        project: impl Into<IdOr<Project>>,
        buildings: impl IntoIterator<Item = IdOr<Building>>,
        ifc: &mut IFC,
    ) -> Self {
        self.relating_object = project.into().or_insert(ifc).id().into();

        self.related_objects.0 = buildings
            .into_iter()
            .map(|id_or| id_or.or_insert(ifc).id())
            .collect();

        self
    }
}

impl RootBuilder for RelAggregates {
    fn root_mut(&mut self) -> &mut Root {
        &mut self.root
    }
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
