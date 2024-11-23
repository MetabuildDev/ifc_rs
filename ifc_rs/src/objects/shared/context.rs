use std::ops::DerefMut;
use std::{fmt::Display, ops::Deref};

use ifc_rs_verify_derive::IfcVerify;

use crate::{
    id::{IdOr, TypedId},
    parser::{
        comma::Comma, list::IfcList, optional::OptionalParameter, string::StringPrimitive,
        IFCParse, IFCParser,
    },
    prelude::*,
};

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
#[derive(IfcVerify)]
pub struct Context {
    #[inherited]
    root: Root,

    /// The type denotes a particular type that indicates the object further.
    /// The use has to be established at the level of instantiable subtypes.
    pub object_type: OptionalParameter<StringPrimitive>,

    /// Long name for the context as used for reference purposes.
    pub long_name: OptionalParameter<StringPrimitive>,

    /// Current project phase, or life-cycle phase of this project. Applicable
    /// values have to be agreed upon by view definitions or implementer agreements.
    pub phase: OptionalParameter<StringPrimitive>,

    /// Context of the representations used within the context. When the
    /// context is a project and it includes shape representations for its
    /// components, one or several geometric representation contexts need
    /// to be included that define e.g. the world coordinate system, the
    /// coordinate space dimensions, and/or the precision factor.
    pub representation_context: IfcList<TypedId<GeometricRepresentationContext>>,

    /// Units globally assigned to measure types used within the context.
    pub units_in_context: OptionalParameter<TypedId<UnitAssigment>>,
}

impl Context {
    pub fn new(root: Root) -> Self {
        Self {
            root,
            object_type: OptionalParameter::omitted(),
            long_name: OptionalParameter::omitted(),
            phase: OptionalParameter::omitted(),
            representation_context: IfcList::empty(),
            units_in_context: OptionalParameter::omitted(),
        }
    }
}

pub trait ContextBuilder: Sized {
    fn context_mut(&mut self) -> &mut Context;

    fn object_type(mut self, object_type: impl Into<StringPrimitive>) -> Self {
        self.context_mut().object_type = object_type.into().into();
        self
    }

    fn long_name(mut self, long_name: impl Into<StringPrimitive>) -> Self {
        self.context_mut().long_name = long_name.into().into();
        self
    }

    fn phase(mut self, phase: impl Into<StringPrimitive>) -> Self {
        self.context_mut().phase = phase.into().into();
        self
    }

    fn add_context(
        mut self,
        representation_context: impl Into<IdOr<GeometricRepresentationContext>>,
        ifc: &mut IFC,
    ) -> Self {
        self.context_mut()
            .representation_context
            .0
            .push(representation_context.into().or_insert(ifc));
        self
    }

    fn unit_assignment(
        mut self,
        unit_assignment: impl Into<IdOr<UnitAssigment>>,
        ifc: &mut IFC,
    ) -> Self {
        self.context_mut().units_in_context = unit_assignment.into().or_insert(ifc).into();
        self
    }
}

impl Deref for Context {
    type Target = Root;

    fn deref(&self) -> &Self::Target {
        &self.root
    }
}

impl DerefMut for Context {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.root
    }
}

impl IFCParse for Context {
    fn parse<'a>() -> impl IFCParser<'a, Self> {
        winnow::seq! {
            Context {
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
