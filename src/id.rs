use std::{fmt::Display, marker::PhantomData};

use winnow::{
    ascii::dec_uint,
    combinator::{alt, preceded},
    Parser,
};

use crate::{
    ifc_type::IfcType,
    parser::{IFCParse, IFCParser},
    IFC,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Id(pub usize);

impl IFCParse for Id {
    fn parse<'a>() -> impl IFCParser<'a, Self>
    where
        Self: Sized,
    {
        preceded("#", dec_uint).map(Self)
    }
}

impl Display for Id {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "#{id}", id = self.0)
    }
}

#[derive(Debug, PartialOrd, Ord)]
pub struct TypedId<T: IfcType> {
    id: Id,
    t: PhantomData<T>,
}

impl<T: IfcType> Copy for TypedId<T> {}

impl<T: IfcType> Eq for TypedId<T> {}

impl<T: IfcType> std::hash::Hash for TypedId<T> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}

impl<T: IfcType> PartialEq for TypedId<T> {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl<T: IfcType> Clone for TypedId<T> {
    fn clone(&self) -> Self {
        Self {
            id: self.id.clone(),
            t: PhantomData,
        }
    }
}

impl<T: IfcType> TypedId<T> {
    pub fn new(id: Id) -> Self {
        Self { id, t: PhantomData }
    }

    pub fn id(&self) -> Id {
        self.id
    }

    pub fn id_or(&self) -> IdOr<T> {
        IdOr::Id(self.id)
    }
}

impl<T: IfcType> Into<IdOr<T>> for TypedId<T> {
    fn into(self) -> IdOr<T> {
        IdOr::Id(self.id)
    }
}

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
}

impl<T: IfcType> IdOr<T> {
    pub(crate) fn into_id(self, ifc: &mut IFC) -> IdOr<T> {
        match self {
            Self::Id(_) => self,
            Self::Custom(t) => ifc.data.insert_new(t).into(),
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
