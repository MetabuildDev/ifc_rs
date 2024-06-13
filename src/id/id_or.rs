use std::fmt::Display;
use winnow::{combinator::alt, Parser};

use crate::{
    ifc_type::IfcType,
    parser::{IFCParse, IFCParser},
    IFC,
};

use super::{Id, TypedId};

#[derive(Debug, Clone)]
pub enum IdOr<T> {
    // e.g. #01
    Id(Id),
    // e.g. .DEGREE.
    Custom(T),
}

impl<T> IdOr<T> {
    pub fn id(&self) -> Id {
        match self {
            Self::Id(id) => *id,
            Self::Custom(_) => panic!("IdOr: called Id on Custom"),
        }
    }

    pub fn mapped_into<I>(self) -> IdOr<I>
    where
        T: Into<I>,
        I: IfcType,
    {
        match self {
            Self::Id(id) => IdOr::Id(id),
            Self::Custom(t) => IdOr::Custom(t.into()),
        }
    }
}

impl<T: IfcType> IdOr<T> {
    pub(crate) fn or_insert(self, ifc: &mut IFC) -> TypedId<T> {
        match self {
            Self::Id(id) => id.into(),
            Self::Custom(t) => ifc.data.insert_new(t),
        }
    }
}

impl<T: IfcType> From<T> for IdOr<T> {
    fn from(value: T) -> Self {
        Self::Custom(value)
    }
}

impl<T> From<Id> for IdOr<T> {
    fn from(value: Id) -> Self {
        Self::Id(value)
    }
}

impl<T: IfcType> From<TypedId<T>> for IdOr<T> {
    fn from(value: TypedId<T>) -> Self {
        IdOr::Id(value.id())
    }
}

impl<T: IFCParse> IFCParse for IdOr<T> {
    fn parse<'a>() -> impl IFCParser<'a, Self>
    where
        Self: Sized,
    {
        alt((Id::parse().map(Self::Id), T::parse().map(Self::Custom)))
    }
}

impl<T: Display> Display for IdOr<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            IdOr::Id(id) => write!(f, "{id}"),
            IdOr::Custom(t) => write!(f, "{t}"),
        }
    }
}
