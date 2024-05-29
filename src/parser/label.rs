use std::fmt::Display;

use winnow::Parser;

use crate::parser::{optional::IFCParse, p_quote_word, IFCParser};

/// A label is the term by which something may be referred to.
/// It is a string which represents the human-interpretable name of something and shall have a natural-language meaning.
///
/// https://standards.buildingsmart.org/IFC/RELEASE/IFC2x3/TC1/HTML/ifcmeasureresource/lexical/ifclabel.htm
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Label(pub String);

impl IFCParse for Label {
    fn parse<'a>() -> impl IFCParser<'a, Self>
    where
        Self: Sized,
    {
        p_quote_word().map(Self)
    }
}

impl Display for Label {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "'{label}'", label = self.0)
    }
}
