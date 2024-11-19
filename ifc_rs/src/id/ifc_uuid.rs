use winnow::{token::take_while, Parser};

use crate::parser::*;

const ALPHABET: &str = "0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz_$";
const BASE64_MASK: u8 = 0b0011_1111;
const BASE16_BYTE_MASK: u8 = 0b1111_1111;

/// An _IfcGloballyUniqueId_ holds an encoded string identifier that is used to uniquely identify an IFC object. An _IfcGloballyUniqueId_ is a Globally Unique Identifier (GUID) which is an auto-generated 128-bit number. Since this identifier is required for all IFC object instances, it is desirable to compress it to reduce overhead. The encoding of the base 64 character set is shown below:
///
/// <pre>
///            1         2         3         4         5         6
///  0123456789012345678901234567890123456789012345678901234567890123
/// "0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz_$";
/// </pre>
///
///
/// The resulting string is a fixed 22 character length string to be exchanged within the IFC exchange file structure.
///
/// The base64 encoding process may differ from common base64 implementations. The following steps are used:
///
///  1. The first byte is encoded in the first two characters
///  2. The remaining bytes are encoded in groups of 3, taking up 4 characters
///
/// As a result, the first character must be either a 0, 1, 2, or 3.
///
/// https://ifc43-docs.standards.buildingsmart.org/IFC/RELEASE/IFC4x3/HTML/lexical/IfcGloballyUniqueId.htm
pub struct IfcGloballyUniqueId(uuid::Uuid);

impl IfcGloballyUniqueId {
    /// translates a given [`uuid::Uuid`] into a [`IfcGloballyUniqueId`]
    fn from_uuid(uuid: uuid::Uuid) -> Self {
        Self(uuid)
    }

    /// creates a new [`IfcGloballyUniqueId`] based on the [`uuid::Uuid::new_v4`] method
    pub fn new_v4() -> Self {
        Self::from_uuid(uuid::Uuid::new_v4())
    }

    /// creates a new [`IfcGloballyUniqueId`] from raw bytes where one bytes equates to two hex
    /// digits
    fn from_base16_bytes(base16_bytes: impl IntoIterator<Item = u8>) -> Self {
        Self(uuid::Uuid::from_bytes(
            base16_bytes
                .into_iter()
                .collect::<Vec<_>>()
                .try_into()
                .expect("input bytes will amount to 16 bytes after decoding"),
        ))
    }

    /// creates a new [`IfcGloballyUniqueId`] from raw bytes where one bytes corresponds to the
    /// index of the base64 encoding from the definition of this type:
    ///
    /// <pre>
    ///            1         2         3         4         5         6
    ///  0123456789012345678901234567890123456789012345678901234567890123
    /// "0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz_$";
    /// </pre>
    fn from_base64_bytes(base64_bytes: impl IntoIterator<Item = u8>) -> Self {
        Self::from_base16_bytes(decode_base16_to_base64(base64_bytes))
    }

    /// returns the shortened base64 version of the uuid string
    pub fn base64_ifc_string(&self) -> String {
        encode_base16_to_base64(self.0.into_bytes())
            .into_iter()
            .map(|base_64_idx| base_64_idx as usize)
            .map(|idx| ALPHABET.chars().nth(idx).unwrap())
            .collect()
    }
}

impl IFCParse for IfcGloballyUniqueId {
    fn parse<'a>() -> impl IFCParser<'a, Self>
    where
        Self: Sized,
    {
        winnow::combinator::delimited(
            "'",
            take_while(22..=22, |c: char| ALPHABET.contains(c))
                .map(|x: &str| {
                    x.chars()
                        .filter_map(|c| ALPHABET.find(c))
                        .map(|pos| pos as u8)
                })
                .map(IfcGloballyUniqueId::from_base64_bytes),
            "'",
        )
    }
}

impl std::fmt::Display for IfcGloballyUniqueId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "'{}'", self.base64_ifc_string())
    }
}

fn decode_base16_to_base64(base64_bytes: impl IntoIterator<Item = u8>) -> Vec<u8> {
    let mut bytes_iter = base64_bytes.into_iter();

    let iter_first = std::iter::from_fn(|| Some([bytes_iter.next()?, bytes_iter.next()?]))
        .take(1)
        .map(combine_first_base64_bytes)
        .collect::<Vec<_>>();

    let iter_chunk_4 = std::iter::from_fn(move || {
        Some([
            bytes_iter.next()?,
            bytes_iter.next()?,
            bytes_iter.next()?,
            bytes_iter.next()?,
        ])
    })
    .map(combine_base64_bytes)
    .flat_map(move |bytes| (0..3).rev().map(move |n| get_base16_bytes(bytes, n)));

    iter_first
        .into_iter()
        .chain(iter_chunk_4)
        .collect::<Vec<_>>()
}

fn get_base16_bytes(combined16: u32, byte: usize) -> u8 {
    (combined16 >> (byte * 8)) as u8 & BASE16_BYTE_MASK
}

fn combine_first_base64_bytes(bytes64: [u8; 2]) -> u8 {
    let [first, second] = bytes64.map(|byte| byte & BASE64_MASK);
    (first << 6) | second
}

fn combine_base64_bytes(bytes64: [u8; 4]) -> u32 {
    bytes64
        .iter()
        .fold(0, |acc, byte| (acc << 6) | (*byte as u32))
}

fn encode_base16_to_base64(base16_bytes: impl IntoIterator<Item = u8>) -> Vec<u8> {
    let mut bytes_iter = base16_bytes.into_iter();

    let first_byte_chars = std::iter::from_fn(|| bytes_iter.next())
        .take(1)
        .flat_map(|byte| (0..2).rev().map(move |n| get_base64_bit(byte as u32, n)))
        .collect::<Vec<_>>();

    let iter_chunk_3 = std::iter::from_fn(move || {
        Some([bytes_iter.next()?, bytes_iter.next()?, bytes_iter.next()?])
    });

    let rest_chars = iter_chunk_3
        .flat_map(|bytes| {
            let combined = combine_base16_bytes(bytes);
            (0..4).rev().map(move |n| get_base64_bit(combined, n))
        })
        .collect::<Vec<_>>();

    first_byte_chars
        .into_iter()
        .chain(rest_chars)
        .collect::<Vec<_>>()
}

fn get_base64_bit(combined64: u32, bit: usize) -> u8 {
    (combined64 >> (bit * 6)) as u8 & BASE64_MASK
}

// giving this function more than 4 bytes results in information loss
fn combine_base16_bytes(bytes16: [u8; 3]) -> u32 {
    bytes16
        .iter()
        .fold(0, |acc, byte| (acc << 8) | (*byte as u32))
}

#[test]
fn ifc_docs_testcase_deserializes() {
    let input = "'3t3TDZl_D9NOIWB0BSjzJI'";
    let parsed = IfcGloballyUniqueId::parse().parse(input).unwrap();

    assert_eq!(parsed.to_string(), input);
}

#[test]
fn ifc_docs_testcase() {
    let uuid = uuid::uuid!("f70dd363-bfe3-495d-84a0-2c02dcb7d4d2");
    let base64_uuid = IfcGloballyUniqueId::from_uuid(uuid);

    assert_eq!(
        base64_uuid.base64_ifc_string(),
        String::from("3t3TDZl_D9NOIWB0BSjzJI")
    );
}
