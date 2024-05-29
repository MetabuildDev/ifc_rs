use std::fmt::Display;

use winnow::{combinator::alt, Parser};

use crate::parser::IFCParser;

use super::place_holder::PlaceHolder;

pub trait IFCParse: Display {
    fn parse<'a>() -> impl IFCParser<'a, Self>
    where
        Self: Sized;
}

#[derive(Debug, Clone)]
pub struct OptionalParameter<T: IFCParse>(Option<T>);

impl<T: IFCParse> IFCParse for OptionalParameter<T> {
    fn parse<'a>() -> impl IFCParser<'a, Self> {
        alt((PlaceHolder::parse().map(|_| None), T::parse().map(Some))).map(Self)
    }
}

impl<T: IFCParse> Display for OptionalParameter<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            OptionalParameter(None) => write!(f, "$"),
            OptionalParameter(Some(t)) => write!(f, "{t}"),
        }
    }
}
