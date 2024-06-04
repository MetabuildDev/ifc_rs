use std::{fmt::Display, ops::Deref};

use crate::{
    id::Id,
    parser::{optional::OptionalParameter, p_space_or_comment_surrounded, IFCParse, IFCParser},
};

use super::root::Root;

/// The object type defines the specific information about a type,
/// being common to all occurrences of this type. It refers to the
/// specific level of the well recognized generic - specific - occurrance
/// modeling paradigm. The IfcTypeObject gets assigned to the individual
/// object instances (the occurrences) via the IfcRelDefinesByType relationship.
///
/// https://standards.buildingsmart.org/IFC/DEV/IFC4_2/FINAL/HTML/schema/ifckernel/lexical/ifctypeobject.htm
pub struct TypeObject {
    root: Root,

    /// The attribute optionally defines the data type of the occurrence
    /// object, to which the assigned type object can relate. If not
    /// present, no instruction is given to which occurrence object the
    /// type object is applicable. The following conventions are used:
    ///
    ///   * The IFC entity name of the applicable occurrence using
    ///     the IFC naming convention, CamelCase with IFC prefix
    ///   * It can be optionally followed by the predefined type after
    ///     the separator "/" (forward slash), using uppercase
    ///   * If one type object is applicable to many occurrence objects,
    ///     then those occurrence object names should be separate by
    ///     comma "," forming a comma separated string.
    pub applicable_occurence: OptionalParameter<Id>,

    /// Set list of unique property sets, that are associated with the
    /// object type and are common to all object occurrences referring
    /// to this object type.
    pub has_property_sets: OptionalParameter<Id>,
}

impl Deref for TypeObject {
    type Target = Root;

    fn deref(&self) -> &Self::Target {
        &self.root
    }
}

impl IFCParse for TypeObject {
    fn parse<'a>() -> impl IFCParser<'a, Self> {
        winnow::seq! {
            Self {
                root: Root::parse(),
                _: p_space_or_comment_surrounded(","),
                applicable_occurence: OptionalParameter::parse(),
                _: p_space_or_comment_surrounded(","),
                has_property_sets: OptionalParameter::parse()
            }
        }
    }
}

impl Display for TypeObject {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{},{},{}",
            self.root, self.applicable_occurence, self.has_property_sets
        )
    }
}

#[cfg(test)]
mod test {
    use winnow::Parser;

    use crate::objects::shared::type_object::TypeObject;
    use crate::parser::IFCParse;

    #[test]
    fn type_object_round_trip() {
        let example = "'2aG1gZj7PD2PztLOx2$IVX',#2,'Double Brick - 270',$,$,$";

        let type_object: TypeObject = TypeObject::parse().parse(example).unwrap();
        let str_type_object = type_object.to_string();

        assert_eq!(example, str_type_object);
    }
}
