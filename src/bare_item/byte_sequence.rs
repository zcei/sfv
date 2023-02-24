use std::ops::Deref;

use data_encoding::BASE64;

use super::SerializeBareItem;

/// Byte Sequences can be conveyed in Structured Fields.
///
/// The ABNF for a Byte Sequence is:
/// ```abnf,ignore,no_run
/// sf-binary = ":" *(base64) ":"
/// base64    = ALPHA / DIGIT / "+" / "/" / "="
/// ```
#[derive(Debug, PartialEq, Clone)]
pub struct BareItemByteSeq(pub(crate) Vec<u8>);

impl From<&[u8]> for BareItemByteSeq {
    fn from(value: &[u8]) -> Self {
        BareItemByteSeq(value.to_vec())
    }
}

impl From<Vec<u8>> for BareItemByteSeq {
    fn from(value: Vec<u8>) -> Self {
        BareItemByteSeq(value)
    }
}

impl Deref for BareItemByteSeq {
    type Target = [u8];
    fn deref(&self) -> &Self::Target {
        self.0.as_slice()
    }
}

impl SerializeBareItem<&[u8]> for BareItemByteSeq {
    fn serialize_ref(value: &[u8], output: &mut String) {
        // https://httpwg.org/specs/rfc8941.html#ser-binary

        output.push(':');
        let encoded = BASE64.encode(value.as_ref());
        output.push_str(&encoded);
        output.push(':');
    }
}
