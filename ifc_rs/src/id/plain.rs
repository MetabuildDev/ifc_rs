use std::fmt::Display;

use winnow::{ascii::dec_uint, combinator::preceded, Parser};

use crate::{
    parser::{IFCParse, IFCParser},
    prelude::*,
};

use super::TypedId;

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

impl<T: IfcType> From<TypedId<T>> for Id {
    fn from(value: TypedId<T>) -> Self {
        value.id()
    }
}
