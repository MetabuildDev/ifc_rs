pub mod bool;
pub mod comma;
pub mod geometry;
pub mod ifc_float;
pub mod ifc_integer;
pub mod label;
pub mod list;
pub mod optional;
pub mod place_holder;
pub mod timestamp;

use std::fmt::Display;

use optional::OptionalParameter;
use winnow::ascii::*;
use winnow::combinator::*;
use winnow::token::*;
use winnow::{error::ErrorKind, Parser};

pub trait IFCParser<'a, T>: Parser<&'a str, T, ErrorKind> {}
impl<'a, T, P: Parser<&'a str, T, ErrorKind>> IFCParser<'a, T> for P {}

pub trait IFCParse: Display {
    fn parse<'a>() -> impl IFCParser<'a, Self>
    where
        Self: Sized;

    fn parse_any<'a>() -> impl IFCParser<'a, Box<dyn Display>>
    where
        Self: Sized + 'static,
    {
        Self::parse().map(|s: Self| Box::new(s) as Box<dyn Display>)
    }

    fn fallback<'a>() -> impl IFCParser<'a, OptionalParameter<Self>>
    where
        Self: Sized,
    {
        fail
    }
}

pub(crate) fn p_ident<'a>() -> impl IFCParser<'a, String> {
    take_while(.., |c: char| {
        c.is_alphanumeric() || ['_', '-', '.', ':'].contains(&c)
    })
    .map(|x: &str| x.to_owned())
}

pub(crate) fn p_word_until<'a>(end: char) -> impl IFCParser<'a, String> {
    take_while(.., move |c: char| c != end).map(|x: &str| x.to_owned())
}

pub(crate) fn p_quote_word<'a>() -> impl IFCParser<'a, String> {
    delimited("'", p_word_until('\''), "'")
}

pub(crate) fn p_comment<'a>() -> impl IFCParser<'a, ()> {
    preceded(
        "/*",
        repeat_till(.., any, "*/").map(|(_, _): (Vec<_>, _)| {}),
    )
}

pub(crate) fn p_single_space_or_comment<'a>() -> impl IFCParser<'a, ()> {
    alt((multispace1.map(drop), p_comment()))
}

// This allows spaces and comments at almost every level of the file and also shrinks the
// performance quiet a bit. Maybe that's overkill
pub(crate) fn p_space_or_comment<'a>() -> impl IFCParser<'a, ()> {
    repeat_till(
        ..,
        p_single_space_or_comment(),
        peek(not(p_single_space_or_comment())),
    )
    .map(|(_, _): (Vec<_>, _)| {})
}

pub(crate) fn p_space_or_comment_surrounded<'a, T>(
    p: impl IFCParser<'a, T>,
) -> impl IFCParser<'a, T> {
    delimited(p_space_or_comment(), p, p_space_or_comment())
}
