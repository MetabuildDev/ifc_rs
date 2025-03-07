use std::fmt::Display;

use winnow::Parser;

use crate::parser::{p_quote_word, IFCParse, IFCParser};

// IFC doesn't support UTF8 in all versions and uses a special encoding for things like
// german umlaute. The easiest thing to use it `\X2\<unicode sequence>\X0\`
//
// https://technical.buildingsmart.org/resources/ifcimplementationguidance/string-encoding/
const UMLAUTE_MAP: [(&str, &str); 7] = [
    ("Ä", r#"\X2\00C4\X0\"#),
    ("ä", r#"\X2\00E4\X0\"#),
    ("Ö", r#"\X2\00D6\X0\"#),
    ("ö", r#"\X2\00F6\X0\"#),
    ("Ü", r#"\X2\00DC\X0\"#),
    ("ü", r#"\X2\00FC\X0\"#),
    ("ß", r#"\X2\00DF\X0\"#),
];

fn translate_encode(inp: impl AsRef<str>) -> String {
    let mut value = inp.as_ref().to_owned();
    for (uft8, encoded) in UMLAUTE_MAP {
        value = value.replace(uft8, encoded);
    }
    value
}

fn translate_decode(inp: impl AsRef<str>) -> String {
    let mut value = inp.as_ref().to_owned();
    for (uft8, encoded) in UMLAUTE_MAP {
        value = value.replace(encoded, uft8);
    }
    value
}

/// A label is the term by which something may be referred to.
/// It is a string which represents the human-interpretable name of something and shall have a natural-language meaning.
///
/// https://standards.buildingsmart.org/IFC/RELEASE/IFC2x3/TC1/HTML/ifcmeasureresource/lexical/ifclabel.htm
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct StringPrimitive(pub String);

impl<S: AsRef<str>> From<S> for StringPrimitive {
    fn from(value: S) -> Self {
        Self(translate_decode(value))
    }
}

impl IFCParse for StringPrimitive {
    fn parse<'a>() -> impl IFCParser<'a, Self>
    where
        Self: Sized,
    {
        p_quote_word().map(Self::from)
    }
}

impl Display for StringPrimitive {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "'{label}'", label = translate_encode(self.0.as_str()))
    }
}
