use std::fmt::Display;

use winnow::{
    ascii::dec_uint,
    combinator::{alt, preceded},
    Parser,
};

use crate::parser::{IFCParse, IFCParser};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
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

#[derive(Debug, Clone)]
pub enum IdOr<T> {
    // e.g. #01
    Id(Id),
    // e.g. .DEGREE.
    Custom(T),
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
