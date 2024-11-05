use std::fmt::Display;

use ifc_rs_verify_derive::IfcVerify;
use label::Label;
use optional::OptionalParameter;

use crate::{parser::*, prelude::*, properties::base::comma::Comma};

/// IfcProperty is an abstract generalization for all types of properties that can be associated with IFC objects through the property set mechanism.
///
/// https://standards.buildingsmart.org/IFC/DEV/IFC4_2/FINAL/HTML/schema/ifcpropertyresource/lexical/ifcproperty.htm
#[derive(IfcVerify)]
pub struct PropertyBase {
    /// Name for this property. This label is the significant name string that defines the semantic meaning for the property.
    pub name: OptionalParameter<Label>,
    /// Informative text to explain the property.
    pub description: OptionalParameter<Label>,
}

impl PropertyBase {
    pub fn new(name: Label) -> Self {
        Self {
            name: name.into(),
            description: OptionalParameter::omitted(),
        }
    }
}

pub trait PropertyBuilder: Sized {
    fn property_mut(&mut self) -> &mut PropertyBase;

    fn name(mut self, name: impl Into<Label>) -> Self {
        self.property_mut().name = name.into().into();
        self
    }

    fn description(mut self, description: impl Into<Label>) -> Self {
        self.property_mut().description = description.into().into();
        self
    }
}

impl IFCParse for PropertyBase {
    fn parse<'a>() -> impl IFCParser<'a, Self>
    where
        Self: Sized,
    {
        winnow::seq! {
            Self {
                name: OptionalParameter::parse(),
                _: Comma::parse(),
                description: OptionalParameter::parse(),
            }
        }
    }
}

impl Display for PropertyBase {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{},{}", self.name, self.description)
    }
}

#[cfg(test)]
mod test {
    use winnow::Parser;

    use crate::properties::base::{IFCParse, PropertyBase};

    #[test]
    fn root_round_trip() {
        let example = "'Reference','Reference'";

        let property_base: PropertyBase = PropertyBase::parse().parse(example).unwrap();
        let str_property_base = property_base.to_string();

        assert_eq!(example, str_property_base);
    }
}
