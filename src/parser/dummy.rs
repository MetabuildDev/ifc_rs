use std::fmt::Display;

use crate::ifc_type::IfcType;

use super::IFCParse;

pub(crate) struct Dummy {
    pub s: String,
}

impl IfcType for Dummy {}

impl IFCParse for Dummy {
    fn parse<'a>() -> impl super::IFCParser<'a, Self>
    where
        Self: Sized,
    {
        #[cfg(test)]
        {
            winnow::combinator::fail
        }

        #[cfg(not(test))]
        {
            use super::p_space_or_comment_surrounded;
            use winnow::{combinator::repeat_till, token::any, Parser};

            repeat_till(.., any, p_space_or_comment_surrounded(";"))
                .map(|(s, _): (String, _)| Self { s })
        }
    }
}

impl Display for Dummy {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{};", self.s)
    }
}
