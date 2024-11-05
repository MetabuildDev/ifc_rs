mod deserialize;
mod serialize;

use crate::parser::optional::OptionalParameter;
use crate::parser::string::StringPrimitive;
use crate::parser::{p_space_or_comment, IFCParse, IFCParser};
use crate::prelude::*;
use std::str::FromStr;

use ifc_rs_verify_derive::IfcVerify;
use strum::{Display, EnumString, VariantNames};
use winnow::combinator::{alt, delimited};
use winnow::Parser;

/// A role which is performed by an actor, either a person, an organization or
/// a person related to an organization.
///
/// https://standards.buildingsmart.org/IFC/RELEASE/IFC2x3/TC1/HTML/ifcactorresource/lexical/ifcactorrole.htm
#[derive(Debug, Clone, IfcVerify)]
pub struct ActorRole {
    /// The name of the role played by an actor. If the Role has value USERDEFINED, then
    /// the user defined role shall be provided as a value of the attribute UserDefinedRole.
    pub role: Role,
    /// Allows for specification of user defined roles beyond the
    /// enumeration values provided by Role attribute of type IfcRoleEnum.
    /// When a value is provided for attribute UserDefinedRole in parallel
    /// the attribute Role shall have enumeration value USERDEFINED.
    pub user_defined_role: OptionalParameter<StringPrimitive>,
    /// A textual description relating the nature of the role played by an actor.
    pub description: OptionalParameter<StringPrimitive>, // TODO: Text
}

/// Roles which may be played by an actor.
///
/// https://standards.buildingsmart.org/IFC/RELEASE/IFC2x3/TC1/HTML/ifcactorresource/lexical/ifcroleenum.htm
#[derive(Debug, EnumString, VariantNames, Display, Clone, Copy)]
pub enum Role {
    #[strum(to_string = ".SUPPLIER.")]
    Supplier,
    #[strum(to_string = ".MANUFACTURER.")]
    Manufacturer,
    #[strum(to_string = ".CONTRACTOR.")]
    Contractor,
    #[strum(to_string = ".SUBCONTRACTOR.")]
    Subcontractor,
    #[strum(to_string = ".ARCHITECT.")]
    Architect,
    #[strum(to_string = ".STRUCTURALENGINEER.")]
    StructuralEngineer,
    #[strum(to_string = ".COSTENGINEER.")]
    CostEngineer,
    #[strum(to_string = ".CLIENT.")]
    Clien,
    #[strum(to_string = ".BUILDINGOWNER.")]
    BuildingOwner,
    #[strum(to_string = ".BUILDINGOPERATOR.")]
    BuildingOperator,
    #[strum(to_string = ".MECHANICALENGINEER.")]
    MechanicalEngineer,
    #[strum(to_string = ".ELECTRICALENGINEER.")]
    ElectricalEngineer,
    #[strum(to_string = ".PROJECTMANAGER.")]
    ProjectManager,
    #[strum(to_string = ".FACILITIESMANAGER.")]
    FacilitiesManager,
    #[strum(to_string = ".CIVILENGINEER.")]
    CivilEngineer,
    #[strum(to_string = ".COMISSIONENGINEER.")]
    ComissionEngineer,
    #[strum(to_string = ".ENGINEER.")]
    Engineer,
    #[strum(to_string = ".OWNER.")]
    Owner,
    #[strum(to_string = ".CONSULTANT.")]
    Consultant,
    #[strum(to_string = ".CONSTRUCTIONMANAGER.")]
    ConstructionManager,
    #[strum(to_string = ".FIELDCONSTRUCTIONMANAGER.")]
    FieldConstructionManager,
    #[strum(to_string = ".RESELLER.")]
    Reseller,
    #[strum(to_string = ".USERDEFINED.")]
    UserDefined,
}

impl IFCParse for Role {
    fn parse<'a>() -> impl IFCParser<'a, Self> {
        let variants: [&str; Self::VARIANTS.len()] =
            Self::VARIANTS.try_into().expect("statically known");

        delimited(
            p_space_or_comment(),
            alt(variants
                .map(|v| (v, Self::from_str(v).expect("valid Role")))
                .map(|(k, v)| k.map(move |_| v))),
            p_space_or_comment(),
        )
    }
}

impl IfcType for ActorRole {}
