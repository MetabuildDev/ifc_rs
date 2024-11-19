use std::fmt::Display;

use comma::Comma;
use ifc_rs_verify_derive::IfcVerify;
use optional::OptionalParameter;
use string::StringPrimitive;

use crate::{
    id::{IdOr, TypedId},
    parser::*,
    prelude::*,
};

/// IfcRoot is the most abstract and root class for all entity definitions
/// that roots in the kernel or in subsequent layers of the IFC specification.
/// It is therefore the common supertype of all IFC entities, beside those
/// defined in an IFC resource schema. All entities that are subtypes of
/// IfcRoot can be used independently, whereas resource schema entities,
/// that are not subtypes of IfcRoot, are not supposed to be independent entities.
///
/// https://standards.buildingsmart.org/IFC/DEV/IFC4_2/FINAL/HTML/schema/ifckernel/lexical/ifcroot.htm
#[derive(IfcVerify)]
pub struct Root {
    /// Assignment of a globally unique identifier within the entire software world.
    pub global_id: IfcGloballyUniqueId,

    /// Assignment of the information about the current ownership of that object,
    /// including owning actor, application, local identification and information
    /// captured about the recent changes of the object.
    pub owner_history: OptionalParameter<TypedId<OwnerHistory>>,

    /// Optional name for use by the participating software systems or users.
    /// For some subtypes of IfcRoot the insertion of the Name attribute may
    /// be required. This would be enforced by a where rule.
    pub name: OptionalParameter<StringPrimitive>,

    /// Optional description, provided for exchanging informative comments.
    pub description: OptionalParameter<StringPrimitive>,
}

impl Root {
    pub fn new(name: StringPrimitive) -> Self {
        Self {
            global_id: IfcGloballyUniqueId::new_v4(),
            owner_history: OptionalParameter::omitted(),
            name: name.into(),
            description: OptionalParameter::omitted(),
        }
    }
}

pub trait RootBuilder: Sized {
    fn root_mut(&mut self) -> &mut Root;

    fn owner_history(
        mut self,
        owner_history: impl Into<IdOr<OwnerHistory>>,
        ifc: &mut IFC,
    ) -> Self {
        self.root_mut().owner_history = owner_history.into().or_insert(ifc).into();
        self
    }

    fn name(mut self, name: impl Into<StringPrimitive>) -> Self {
        self.root_mut().name = name.into().into();
        self
    }

    fn description(mut self, description: impl Into<StringPrimitive>) -> Self {
        self.root_mut().description = description.into().into();
        self
    }
}

impl IFCParse for Root {
    fn parse<'a>() -> impl IFCParser<'a, Self> {
        winnow::seq! {
            Self {
                global_id: IfcGloballyUniqueId::parse(),
                _: Comma::parse(),
                owner_history: OptionalParameter::parse(),
                _: Comma::parse(),
                name: OptionalParameter::parse(),
                _: Comma::parse(),
                description: OptionalParameter::parse(),
            }
        }
    }
}

impl Display for Root {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{},{},{},{}",
            self.global_id, self.owner_history, self.name, self.description
        )
    }
}

#[cfg(test)]
mod test {
    use winnow::Parser;

    use crate::objects::shared::root::{IFCParse, Root};

    #[test]
    fn root_round_trip() {
        let example = "'2aG1gZj7PD2PztLOx2$IVX',#2,'Double Brick - 270',$";

        let root: Root = Root::parse().parse(example).unwrap();
        let str_root = root.to_string();

        assert_eq!(example, str_root);
    }
}
