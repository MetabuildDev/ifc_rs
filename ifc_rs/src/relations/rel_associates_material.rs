use std::{fmt::Display, ops::Deref};

use ifc_rs_verify_derive::IfcVerify;

use crate::{
    id::{Id, IdOr},
    ifc_type::{IfcType, IfcVerify},
    parser::{comma::Comma, label::Label, p_space_or_comment_surrounded, IFCParse, IFCParser},
    prelude::*,
};

/// Material set usages & material sets which can be related to.
pub trait RelatableMaterial: IfcType {}

/// Objects which can be related to materials
pub trait MaterialRelatable: IfcType {}

/// The aggregation relationship IfcRelAggregates is a special type of
/// the general composition/decomposition (or whole/part) relationship
/// IfcRelDecomposes. The aggregation relationship can be applied to
/// all subtypes of IfcObjectDefinition.
///
/// https://standards.buildingsmart.org/IFC/DEV/IFC4_2/FINAL/HTML/link/ifcrelassociatesmaterial.htm
#[derive(IfcVerify)]
pub struct RelAssociatesMaterial {
    rel_associates: RelAssociates,

    /// Material definition assigned to the elements or element types.
    #[ifc_types(
        MaterialConstituentSet,
        MaterialLayerSetUsage,
        MaterialLayerSet,
        Material
    )]
    pub relating_material: Id,
}

impl RelAssociatesMaterial {
    pub fn new<R: RelatableMaterial>(
        name: impl Into<Label>,
        material_layer_set: impl Into<IdOr<R>>,
        ifc: &mut IFC,
    ) -> Self {
        Self {
            rel_associates: RelAssociates::new(Root::new(name.into())),
            relating_material: material_layer_set.into().or_insert(ifc).id(),
        }
    }

    pub fn is_related_to(&self, id: impl Into<Id>) -> bool {
        self.rel_associates.related_objects.0.contains(&id.into())
    }
}

impl<T: MaterialRelatable> RelAssociatesBuilder<T> for RelAssociatesMaterial {
    fn rel_associates_mut(&mut self) -> &mut RelAssociates {
        &mut self.rel_associates
    }
}

impl RootBuilder for RelAssociatesMaterial {
    fn root_mut(&mut self) -> &mut Root {
        &mut self.rel_associates
    }
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
                relating_material: Id::parse(),

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
