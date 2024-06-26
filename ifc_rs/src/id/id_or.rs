use std::fmt::Display;
use winnow::{combinator::alt, Parser};

use crate::{
    parser::{IFCParse, IFCParser},
    prelude::*,
};

use super::{Id, TypedId};

#[derive(Debug, Clone)]
pub enum IdOr<T: IfcType> {
    // e.g. #01
    Id(TypedId<T>),
    // e.g. .DEGREE.
    Custom(T),
}

impl<T: IfcType> IdOr<T> {
    pub fn id(&self) -> Option<TypedId<T>> {
        match self {
            Self::Id(id) => Some(*id),
            Self::Custom(_) => None,
        }
    }

    pub fn custom(&self) -> Option<&T> {
        match self {
            Self::Id(_) => None,
            Self::Custom(t) => Some(t),
        }
    }

    pub(crate) fn or_insert(self, ifc: &mut IFC) -> TypedId<T> {
        match self {
            Self::Id(id) => id,
            Self::Custom(t) => ifc.data.insert_new(t),
        }
    }
}

impl<T: IfcType> From<T> for IdOr<T> {
    fn from(value: T) -> Self {
        Self::Custom(value)
    }
}

impl<T: IfcType> From<Id> for IdOr<T> {
    fn from(value: Id) -> Self {
        Self::Id(TypedId::new(value))
    }
}

impl<T: IfcType> From<TypedId<T>> for IdOr<T> {
    fn from(value: TypedId<T>) -> Self {
        IdOr::Id(value)
    }
}

impl<T: IFCParse + IfcType> IFCParse for IdOr<T> {
    fn parse<'a>() -> impl IFCParser<'a, Self>
    where
        Self: Sized,
    {
        alt((
            Id::parse().map(|id| Self::Id(TypedId::new(id))),
            T::parse().map(Self::Custom),
        ))
    }
}

impl<T: Display + IfcType> Display for IdOr<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            IdOr::Id(id) => write!(f, "{id}"),
            IdOr::Custom(t) => write!(f, "{t}"),
        }
    }
}
