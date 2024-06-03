use std::{fmt::Display, ops::Deref};

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

impl<T: IFCParse> OptionalParameter<T> {
    pub fn is_custom(&self) -> bool {
        match self {
            OptionalParameter::Custom(_) => true,
            _ => false,
        }
    }
}

impl<T: IFCParse> Deref for OptionalParameter<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        match self {
            Self::Custom(t) => t,
            _ => panic!("called deref on non-custom variant"),
        }
    }
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
