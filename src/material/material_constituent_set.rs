use std::fmt::Display;

use ifc_verify_derive::IfcVerify;

use crate::{
    id::{IdOr, TypedId},
    ifc_type::{IfcType, IfcVerify},
    parser::{
        comma::Comma, label::Label, list::IfcList, optional::OptionalParameter,
        p_space_or_comment_surrounded, IFCParse, IFCParser,
    },
    prelude::*,
    relations::rel_associates_material::RelatableMaterial,
};

/// IfcMaterialConstituentSet is a collection of individual material
/// constituents, each assigning a material to a part of an element. The parts
/// are only identified by a keyword (as opposed to an IfcMaterialLayerSet or
/// IfcMaterialProfileSet where each part has an individual shape parameter
/// (layer thickness or layer profile).
///
/// https://standards.buildingsmart.org/IFC/DEV/IFC4_2/FINAL/HTML/schema/ifcmaterialresource/lexical/ifcmaterialconstituentset.htm
#[derive(IfcVerify)]
pub struct MaterialConstituentSet {
    /// The name by which the constituent set is known.
    pub name: OptionalParameter<Label>,

    /// Definition of the material constituent set in descriptive terms.
    pub description: OptionalParameter<Label>,

    /// The name by which the IfcMaterialLayerSet is known.
    pub material_constituents: IfcList<TypedId<MaterialConstituent>>,
}

impl MaterialConstituentSet {
    pub fn new() -> Self {
        Self {
            name: OptionalParameter::omitted(),
            description: OptionalParameter::omitted(),
            material_constituents: IfcList::empty(),
        }
    }

    pub fn name(mut self, name: impl Into<Label>) -> Self {
        self.name = name.into().into();
        self
    }

    pub fn description(mut self, description: impl Into<Label>) -> Self {
        self.description = description.into().into();
        self
    }

    pub fn add_constituent(
        mut self,
        constituent: impl Into<IdOr<MaterialConstituent>>,
        ifc: &mut IFC,
    ) -> Self {
        self.material_constituents
            .0
            .push(constituent.into().or_insert(ifc));
        self
    }
}

impl IFCParse for MaterialConstituentSet {
    fn parse<'a>() -> impl IFCParser<'a, Self> {
        winnow::seq! {
            Self {
                _: p_space_or_comment_surrounded("IFCMATERIALCONSTITUENTSET("),

                name: OptionalParameter::parse(),
                _: Comma::parse(),
                description: OptionalParameter::parse(),
                _: Comma::parse(),
                material_constituents: IfcList::parse(),

                _: p_space_or_comment_surrounded(");"),
            }
        }
    }
}

impl Display for MaterialConstituentSet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "IFCMATERIALCONSTITUENTSET({},{},{});",
            self.name, self.description, self.material_constituents,
        )
    }
}

impl IfcType for MaterialConstituentSet {}
impl RelatableMaterial for MaterialConstituentSet {}

#[cfg(test)]
mod test {
    use winnow::Parser;

    use super::MaterialConstituentSet;
    use crate::parser::IFCParse;

    #[test]
    fn material_layer_set_round_trip() {
        let example = "IFCMATERIALCONSTITUENTSET('Constituent Set for Window',$,(#104,#105));";

        let parsed = MaterialConstituentSet::parse().parse(example).unwrap();
        let str = parsed.to_string();

        assert_eq!(example, str);
    }
}
