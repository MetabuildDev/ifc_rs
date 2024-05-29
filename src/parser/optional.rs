use std::fmt::Display;

use winnow::{combinator::alt, Parser};

use super::place_holder::PlaceHolder;
use crate::{
    id::Id,
    parser::{p_id, IFCParser},
};

pub trait OptionalParse: Display {
    fn opt_parse<'a>() -> impl IFCParser<'a, Self>
    where
        Self: Sized;
}

#[derive(Debug, Clone)]
pub enum OptionalParameter<T: OptionalParse> {
    // '$'
    PlaceHolder,
    // e.g. #01
    Id(Id),
    // e.g. .DEGREE.
    Custom(T),
}

impl<T: OptionalParse> OptionalParameter<T> {
    pub(crate) fn opt_parse<'a>() -> impl IFCParser<'a, Self> {
        alt((
            PlaceHolder::parse().map(|_| Self::PlaceHolder),
            p_id().map(|v| Self::Id(v)),
            T::opt_parse().map(|v| Self::Custom(v)),
        ))
    }
}

impl<T: OptionalParse> Display for OptionalParameter<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            OptionalParameter::PlaceHolder => write!(f, "$"),
            OptionalParameter::Id(id) => write!(f, "#{id}"),
            OptionalParameter::Custom(t) => write!(f, "{t}"),
        }
    }
}
