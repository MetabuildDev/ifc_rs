use std::fmt::Display;

use crate::{
    id::Id,
    ifc_type::IfcType,
    parser::{
        comma::Comma, label::Label, list::IfcList, optional::OptionalParameter,
        p_space_or_comment_surrounded, IFCParse, IFCParser,
    },
};

/// The IfcMaterialLayerSet is a designation by which materials of an element
/// constructed of a number of material layers is known and through which the
/// relative positioning of individual layers can be expressed.
///
/// https://standards.buildingsmart.org/IFC/DEV/IFC4_2/FINAL/HTML/link/ifcmateriallayerset.htm
pub struct MaterialLayerSet {
    /// Identification of the IfcMaterialLayer’s from which the
    /// IfcMaterialLayerSet is composed.
    pub material_layers: IfcList<Id>,

    /// The name by which the IfcMaterialLayerSet is known.
    pub layer_set_name: OptionalParameter<Label>,

    /// Definition of the IfcMaterialLayerSet in descriptive terms.
    pub description: OptionalParameter<Label>,
}

impl IFCParse for MaterialLayerSet {
    fn parse<'a>() -> impl IFCParser<'a, Self> {
        winnow::seq! {
            Self {
                _: p_space_or_comment_surrounded("IFCMATERIALLAYERSET("),

                material_layers: IfcList::parse(),
                _: Comma::parse(),
                layer_set_name: OptionalParameter::parse(),
                _: Comma::parse(),
                description: OptionalParameter::parse(),

                _: p_space_or_comment_surrounded(");"),
            }
        }
    }
}

impl Display for MaterialLayerSet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "IFCMATERIALLAYERSET({},{},{});",
            self.material_layers, self.layer_set_name, self.description,
        )
    }
}

impl IfcType for MaterialLayerSet {}

#[cfg(test)]
mod test {
    use winnow::Parser;

    use super::MaterialLayerSet;
    use crate::parser::IFCParse;

    #[test]
    fn material_layer_set_round_trip() {
        let example = "IFCMATERIALLAYERSET((#40,#41,#42),'Double Brick - 270',$);";

        let parsed: MaterialLayerSet = MaterialLayerSet::parse().parse(example).unwrap();
        let str = parsed.to_string();

        assert_eq!(example, str);
    }
}
