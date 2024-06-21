use std::fmt::Display;

use winnow::{combinator::alt, Parser};

use crate::parser::{IFCParse, IFCParser};

use super::place_holder::{Inherited, Omitted};

#[derive(Debug, Clone)]
pub enum OptionalParameter<T: IFCParse> {
    Omitted(Omitted),
    Inherited(Inherited),
    Custom(T),
}

impl<T: IFCParse> OptionalParameter<T> {
    pub fn omitted() -> Self {
        Self::Omitted(Omitted)
    }

    pub fn inherited() -> Self {
        Self::Inherited(Inherited)
    }

    pub fn is_custom(&self) -> bool {
        matches!(self, OptionalParameter::Custom(_))
    }

    pub fn custom(&self) -> Option<&T> {
        match self {
            OptionalParameter::Custom(t) => Some(t),
            _ => None,
        }
    }

    pub fn custom_mut(&mut self) -> Option<&mut T> {
        match self {
            OptionalParameter::Custom(t) => Some(t),
            _ => None,
        }
    }

    pub fn is_inherited(&self) -> bool {
        matches!(self, OptionalParameter::Inherited(_))
    }

    pub fn is_omitted(&self) -> bool {
        matches!(self, OptionalParameter::Omitted(_))
    }
}

impl<T: IFCParse> From<Option<T>> for OptionalParameter<T> {
    fn from(value: Option<T>) -> Self {
        match value {
            Some(t) => Self::Custom(t),
            None => Self::omitted(),
        }
    }
}

impl<T: IFCParse> From<T> for OptionalParameter<T> {
    fn from(value: T) -> Self {
        Self::Custom(value)
    }
}

impl<T: IFCParse> IFCParse for OptionalParameter<T> {
    fn parse<'a>() -> impl IFCParser<'a, Self> {
        alt((
            Omitted::parse().map(Self::Omitted),
            Inherited::parse().map(Self::Inherited),
            T::parse().map(Self::Custom),
            T::fallback(),
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
