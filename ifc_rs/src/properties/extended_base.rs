use std::fmt::Display;

use comma::Comma;
use ifc_rs_verify_derive::IfcVerify;
use list::IfcList;
use optional::OptionalParameter;
use string::StringPrimitive;

use crate::{parser::*, prelude::*};

/// The IfcExtendedProperties is an abstract supertype of all extensible property collections that
/// are applicable to certain characterized entities. Instantiable subtypes of
/// IfcExtendedProperties assign the property collection to a particular characterized entity.
///
/// https://ifc43-docs.standards.buildingsmart.org/IFC/RELEASE/IFC4x3/HTML/lexical/IfcExtendedProperties.htm
#[derive(IfcVerify)]
pub struct ExtendedPropertyBase {
    /// The name given to the set of properties.
    pub name: OptionalParameter<StringPrimitive>,
    /// Description for the set of properties.
    pub description: OptionalParameter<StringPrimitive>,
    /// The set of properties provided for this extended property collection.
    pub properties: IfcList<Id>,
}

impl ExtendedPropertyBase {
    pub fn new(name: StringPrimitive, properties: impl IntoIterator<Item = Id>) -> Self {
        Self {
            name: name.into(),
            description: OptionalParameter::omitted(),
            properties: IfcList(properties.into_iter().collect()),
        }
    }
}

pub trait ExtendedPropertyBuilder: Sized {
    fn property_mut(&mut self) -> &mut ExtendedPropertyBase;

    fn name(mut self, name: impl Into<StringPrimitive>) -> Self {
        self.property_mut().name = name.into().into();
        self
    }

    fn description(mut self, description: impl Into<StringPrimitive>) -> Self {
        self.property_mut().description = description.into().into();
        self
    }

    fn properties(mut self, property: impl Into<Id>) -> Self {
        self.property_mut().properties.0.push(property.into());
        self
    }
}

impl IFCParse for ExtendedPropertyBase {
    fn parse<'a>() -> impl IFCParser<'a, Self>
    where
        Self: Sized,
    {
        winnow::seq! {
            Self {
                name: OptionalParameter::parse(),
                _: Comma::parse(),
                description: OptionalParameter::parse(),
                _: Comma::parse(),
                properties: IfcList::parse()
            }
        }
    }
}

impl Display for ExtendedPropertyBase {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{},{},{}", self.name, self.description, self.properties)
    }
}

#[cfg(test)]
mod test {
    use winnow::Parser;

    use super::{ExtendedPropertyBase, IFCParse};

    #[test]
    fn extended_property_base_round_trip() {
        let example = "'Reference','Reference',(#123,#124,#125)";

        let property_base = ExtendedPropertyBase::parse().parse(example).unwrap();
        let str_property_base = property_base.to_string();

        assert_eq!(example, str_property_base);
    }
}
