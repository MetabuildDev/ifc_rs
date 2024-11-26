use std::fmt::Display;

use ifc_rs_verify_derive::IfcVerify;

use crate::{
    id::{IdOr, TypedId},
    parser::{
        comma::Comma, optional::OptionalParameter, p_space_or_comment_surrounded,
        real::RealPrimitive, string::StringPrimitive, IFCParse, IFCParser,
    },
    prelude::*,
};

/// IfcMaterialConstituent is a single and identifiable part of an element
/// which is constructed of a number of part (one or more) each having an
/// individual material. The association of the material constituent to the
/// part is provided by a keyword as value of the Name attribute. In order to
/// identify and distinguish the part of the shape representation to which the
/// material constituent applies the IfcProductDefinitionShape of the element
/// has to include instances of IfcShapeAspect, using the same keyword for
/// their Name attribute.
///
/// https://standards.buildingsmart.org/IFC/DEV/IFC4_2/FINAL/HTML/schema/ifcmaterialresource/lexical/ifcmaterialconstituent.htm
#[derive(IfcVerify)]
pub struct MaterialConstituent {
    /// The name by which the material constituent is known.
    pub name: OptionalParameter<StringPrimitive>,

    /// Definition of the material constituent in descriptive terms.
    pub description: OptionalParameter<StringPrimitive>, // TODO: text

    /// Reference to the material from which the constituent is constructed.
    pub material: TypedId<Material>,

    /// Optional provision of a fraction of the total amount (volume or weight)
    /// that applies to the IfcMaterialConstituentSet that is contributed by
    /// this IfcMaterialConstituent.
    pub fraction: OptionalParameter<RealPrimitive>, // TODO: Normalized ratio measure

    /// Category of the material constituent, e.g. the role it has in the
    /// constituent set it belongs to.
    pub category: OptionalParameter<StringPrimitive>,
}

impl MaterialConstituent {
    pub fn new(material: TypedId<Material>) -> Self {
        Self {
            name: OptionalParameter::omitted(),
            description: OptionalParameter::omitted(),
            material,
            category: OptionalParameter::omitted(),
            fraction: OptionalParameter::omitted(),
        }
    }

    pub fn material(mut self, material: impl Into<IdOr<Material>>, ifc: &mut IFC) -> Self {
        self.material = material.into().or_insert(ifc).id().into();
        self
    }

    pub fn name(mut self, name: impl Into<StringPrimitive>) -> Self {
        self.name = name.into().into();
        self
    }

    pub fn description(mut self, description: impl Into<StringPrimitive>) -> Self {
        self.description = description.into().into();
        self
    }

    pub fn category(mut self, category: impl Into<StringPrimitive>) -> Self {
        self.category = category.into().into();
        self
    }

    pub fn fraction(mut self, fraction: f64) -> Self {
        self.fraction = RealPrimitive(fraction).into();
        self
    }
}

impl IFCParse for MaterialConstituent {
    fn parse<'a>() -> impl IFCParser<'a, Self> {
        winnow::seq! {
            MaterialConstituent {
                _: p_space_or_comment_surrounded("IFCMATERIALCONSTITUENT("),

                name: OptionalParameter::parse(),
                _: Comma::parse(),
                description: OptionalParameter::parse(),
                _: Comma::parse(),
                material: TypedId::parse(),
                _: Comma::parse(),
                fraction: OptionalParameter::parse(),
                _: Comma::parse(),
                category: OptionalParameter::parse(),

                _: p_space_or_comment_surrounded(");"),
            }
        }
    }
}

impl Display for MaterialConstituent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "IFCMATERIALCONSTITUENT({},{},{},{},{});",
            self.name, self.description, self.material, self.fraction, self.category,
        )
    }
}

impl IfcType for MaterialConstituent {}

#[cfg(test)]
mod test {
    use winnow::Parser;

    use super::MaterialConstituent;
    use crate::parser::IFCParse;

    #[test]
    fn material_constituent_round_trip() {
        let example = "IFCMATERIALCONSTITUENT('Framing',$,#106,$,$);";

        let parsed = MaterialConstituent::parse().parse(example).unwrap();
        let str = parsed.to_string();

        assert_eq!(example, str);
    }
}
