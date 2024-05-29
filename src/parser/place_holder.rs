use winnow::combinator::alt;

use super::IFCParser;

pub struct PlaceHolder;

impl PlaceHolder {
    pub fn parse<'a>() -> impl IFCParser<'a, &'a str> {
        alt(("$", "*"))
    }
}
