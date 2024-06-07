use std::{fmt::Display, ops::Deref};

use crate::{
    id::Id,
    ifc_type::IfcType,
    parser::{
        comma::Comma, optional::OptionalParameter, p_space_or_comment_surrounded, IFCParse,
        IFCParser,
    },
};

use super::shared::rel_associates::RelAssociates;

/// The aggregation relationship IfcRelAggregates is a special type of
/// the general composition/decomposition (or whole/part) relationship
/// IfcRelDecomposes. The aggregation relationship can be applied to
/// all subtypes of IfcObjectDefinition.
///
/// https://standards.buildingsmart.org/IFC/DEV/IFC4_2/FINAL/HTML/link/ifcrelassociatesmaterial.htm
pub struct RelAssociatesMaterial {
    rel_associates: RelAssociates,

    /// Material definition assigned to the elements or element types.
    pub relating_material: OptionalParameter<Id>,
}

impl Deref for RelAssociatesMaterial {
    type Target = RelAssociates;

    fn deref(&self) -> &Self::Target {
        &self.rel_associates
    }
}

impl IFCParse for RelAssociatesMaterial {
    fn parse<'a>() -> impl IFCParser<'a, Self> {
        winnow::seq! {
            Self {
                _: p_space_or_comment_surrounded("IFCRELASSOCIATESMATERIAL("),

                rel_associates: RelAssociates::parse(),
                _: Comma::parse(),
                relating_material: OptionalParameter::parse(),

                _: p_space_or_comment_surrounded(");"),
            }
        }
    }
}

impl Display for RelAssociatesMaterial {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "IFCRELASSOCIATESMATERIAL({},{});",
            self.rel_associates, self.relating_material
        )
    }
}

impl IfcType for RelAssociatesMaterial {}

#[cfg(test)]
mod test {
    use winnow::Parser;

    use super::RelAssociatesMaterial;
    use crate::parser::IFCParse;

    #[test]
    fn rel_associates_material_round_trip() {
        let example = "IFCRELASSOCIATESMATERIAL('1BYoVhjtLADPUZYzipA826',#2,'MatAssoc','Material Associates',(#11),#38);";

        let parsed: RelAssociatesMaterial = RelAssociatesMaterial::parse().parse(example).unwrap();
        let str = parsed.to_string();

        assert_eq!(example, str);
    }
}
