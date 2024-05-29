use std::fmt::Display;

use winnow::{combinator::alt, Parser};

use crate::parser::IFCParser;

use super::place_holder::{Inherited, Omitted};

pub trait IFCParse: Display {
    fn parse<'a>() -> impl IFCParser<'a, Self>
    where
        Self: Sized;
}

#[derive(Debug, Clone)]
pub enum OptionalParameter<T: IFCParse> {
    Omitted(Omitted),
    Inherited(Inherited),
    Custom(T),
}

impl<T: IFCParse> IFCParse for OptionalParameter<T> {
    fn parse<'a>() -> impl IFCParser<'a, Self> {
        alt((
            Omitted::parse().map(Self::Omitted),
            Inherited::parse().map(Self::Inherited),
            T::parse().map(Self::Custom),
        ))
    }
}

impl<T: IFCParse> Display for OptionalParameter<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            OptionalParameter::Omitted(p) => write!(f, "{p}"),
            OptionalParameter::Inherited(p) => write!(f, "{p}"),
            OptionalParameter::Custom(t) => write!(f, "{t}"),
        }
    }
}
