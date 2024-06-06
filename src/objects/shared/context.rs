use std::{fmt::Display, ops::Deref};

use crate::id::Id;
use crate::parser::comma::Comma;
use crate::parser::list::IfcList;
use crate::parser::IFCParse;
use crate::parser::{label::Label, optional::OptionalParameter, IFCParser};

use super::root::Root;

/// IfcContext is the generalization of a project context in which objects,
/// type objects, property sets, and properties are defined. The IfcProject
/// as subtype of IfcContext provides the context for all information on a
/// construction project, it may include one or several IfcProjectLibrary's
/// as subtype of IfcContext to register the included libraries for the project.
/// A library of products that is referenced is declared within the
/// IfcProjectLibrary as the context of that library.
///
/// https://standards.buildingsmart.org/IFC/DEV/IFC4_2/FINAL/HTML/link/ifccontext.htm
pub struct Context {
    root: Root,

    /// The type denotes a particular type that indicates the object further.
    /// The use has to be established at the level of instantiable subtypes.
    pub object_type: OptionalParameter<Label>,

    /// Long name for the context as used for reference purposes.
    pub long_name: OptionalParameter<Label>,

    /// Current project phase, or life-cycle phase of this project. Applicable
    /// values have to be agreed upon by view definitions or implementer agreements.
    pub phase: OptionalParameter<Label>,

    /// Context of the representations used within the context. When the
    /// context is a project and it includes shape representations for its
    /// components, one or several geometric representation contexts need
    /// to be included that define e.g. the world coordinate system, the
    /// coordinate space dimensions, and/or the precision factor.
    pub representation_context: IfcList<Id>,

    /// Units globally assigned to measure types used within the context.
    pub units_in_context: OptionalParameter<Id>,
}

impl Deref for Context {
    type Target = Root;

    fn deref(&self) -> &Self::Target {
        &self.root
    }
}

impl IFCParse for Context {
    fn parse<'a>() -> impl IFCParser<'a, Self> {
        winnow::seq! {
            Self {
                root: Root::parse(),
                _: Comma::parse(),
                object_type: OptionalParameter::parse(),
                _: Comma::parse(),
                long_name: OptionalParameter::parse(),
                _: Comma::parse(),
                phase: OptionalParameter::parse(),
                _: Comma::parse(),
                representation_context: IfcList::parse(),
                _: Comma::parse(),
                units_in_context: OptionalParameter::parse(),
            }
        }
    }
}

impl Display for Context {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{},{},{},{},{},{}",
            self.root,
            self.object_type,
            self.long_name,
            self.phase,
            self.representation_context,
            self.units_in_context,
        )
    }
}
