use std::ops::Deref;

use ifc_rs_verify_derive::IfcVerify;
pub use type_enum::SpaceTypeEnum;

use crate::{
    ifc_type::{IfcType, IfcVerify},
    parser::{label::Label, optional::OptionalParameter},
    prelude::{ElementTypeBuilder, Root, RootBuilder, TypeObject, TypeProduct},
    IFC,
};

use super::shared::{
    element_type::ElementType, type_object::TypeObjectBuilder, type_product::TypeProductBuilder,
};

mod deserialize;
mod serialize;
pub mod type_enum;

/// A space represents an area or volume bounded actually or theoretically.
/// Spaces are areas or volumes that provide for certain functions within a
/// building.
///
/// The IfcSpaceType defines a list of commonly shared defines commonly shared
/// information for occurrences of spaces. The set of shared information may include:
///
/// * common properties within shared property sets
/// * common shape representations
///
/// It is used to define an space specification (i.e. the specific space
/// information, that is common to all occurrences of that space type. Space
/// types may be exchanged without being already assigned to occurrences.
///
/// Note: The space types are often used to represent space catalogues, less
/// so for sharing a common representation map. Space types in a space catalogue
/// share same space classification and a common set of space requirement
/// properties.
///
/// The occurrences of IfcSpaceType are represented by instances of IfcSpace.
///
/// https://standards.buildingsmart.org/IFC/DEV/IFC4_2/FINAL/HTML/schema/ifcproductextension/lexical/ifcspacetype.htm
#[derive(IfcVerify)]
pub struct SpaceType {
    #[inherited]
    element_type: ElementType,

    /// Predefined types to define the particular type of space. There may be
    /// property set definitions available for each predefined type.
    pub predefined_type: SpaceTypeEnum,

    /// Long name for a space type, used for informal purposes. It should be
    /// used, if available, in conjunction with the inherited Name attribute.
    pub long_name: OptionalParameter<Label>,
}

impl SpaceType {
    pub fn new(name: impl Into<Label>, predefined_type: SpaceTypeEnum) -> Self {
        Self {
            element_type: ElementType::new(TypeProduct::new(TypeObject::new(Root::new(
                name.into(),
            )))),
            predefined_type,
            long_name: OptionalParameter::omitted(),
        }
    }
}

impl ElementTypeBuilder for SpaceType {
    fn element_type_mut(&mut self) -> &mut ElementType {
        &mut self.element_type
    }
}

impl TypeProductBuilder for SpaceType {
    fn type_product_mut(&mut self) -> &mut TypeProduct {
        &mut self.element_type
    }
}

impl TypeObjectBuilder for SpaceType {
    fn type_object_mut(&mut self) -> &mut TypeObject {
        &mut self.element_type
    }
}

impl RootBuilder for SpaceType {
    fn root_mut(&mut self) -> &mut Root {
        &mut self.element_type
    }
}

impl Deref for SpaceType {
    type Target = ElementType;

    fn deref(&self) -> &Self::Target {
        &self.element_type
    }
}

impl IfcType for SpaceType {}

#[cfg(test)]
mod test {
    use winnow::Parser;

    use crate::parser::IFCParse;

    use super::SpaceType;

    #[test]
    fn space_type_round_trip() {
        let example = "IFCSPACETYPE('3Hdd9g5lPEL2590fLV6Kl5',#12,'B20FC0oarbeit',$,$,$,(#24363,#24365,#24367,#24788,#24790,#24792,#25209,#25211,#25213,#25630,#25632,#25634,#26051,#26053,#26055,#26472,#26474,#26476,#26893,#26895,#26897,#27314,#27316,#27318,#27735,#27737,#27739,#28156,#28158,#28160,#28577,#28579,#28581,#28998,#29000,#29002,#29419,#29421,#29423,#29840,#29842,#29844,#30261,#30263,#30265,#30704,#30706,#30708,#31125,#31127,#31129,#74133,#74135,#74137,#74554,#74556,#74558,#74975,#74977,#74979,#75396,#75398,#75400,#75817,#75819,#75821,#76238,#76240,#76242,#76659,#76661,#76663,#77080,#77082,#77084,#77501,#77503,#77505,#77922,#77924,#77926,#78343,#78345,#78347,#78764,#78766,#78768,#79185,#79187,#79189,#79606,#79608,#79610,#80027,#80029,#80031,#80448,#80450,#80452,#80869,#80871,#80873,#81334,#81336,#81338,#126471,#126473,#126475,#126892,#126894,#126896,#127313,#127315,#127317,#127734,#127736,#127738,#128155,#128157,#128159,#128598,#128600,#128602,#129019,#129021,#129023,#129440,#129442,#129444,#129861,#129863,#129865,#130282,#130284,#130286,#130703,#130705,#130707,#131124,#131126,#131128,#131545,#131547,#131549,#131966,#131968,#131970,#132387,#132389,#132391,#132808,#132810,#132812,#133229,#133231,#133233,#133650,#133652,#133654,#134071,#134073,#134075,#134492,#134494,#134496,#134913,#134915,#134917,#135334,#135336,#135338,#179231,#179233,#179235,#179652,#179654,#179656,#180073,#180075,#180077,#180494,#180496,#180498,#180915,#180917,#180919,#181336,#181338,#181340,#181757,#181759,#181761,#182178,#182180,#182182,#182599,#182601,#182603,#183020,#183022,#183024,#183441,#183443,#183445,#183862,#183864,#183866,#184283,#184285,#184287,#184704,#184706,#184708,#185125,#185127,#185129,#185546,#185548,#185550,#185967,#185969,#185971,#186388,#186390,#186392,#186809,#186811,#186813,#187230,#187232,#187234,#187673,#187675,#187677,#188094,#188096,#188098,#226717,#226719,#226721,#227192,#227194,#227196,#227667,#227669,#227671),'D19E726A-16F6-4E54-2149-02955F194BC5',$,.NOTDEFINED.,$);";

        let space_type = SpaceType::parse().parse(example).unwrap();
        let str_space_type = space_type.to_string();

        assert_eq!(example, str_space_type);
    }
}
