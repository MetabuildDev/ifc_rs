use std::fmt::Display;

use winnow::Parser;

use crate::parser::IFCParse;

use super::p_space_or_comment_surrounded;

pub struct Comma;

impl IFCParse for Comma {
    fn parse<'a>() -> impl super::IFCParser<'a, Self>
    where
        Self: Sized,
    {
        p_space_or_comment_surrounded(",").map(|_| Self)
    }
}

impl Display for Comma {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, ",")
    }
}
