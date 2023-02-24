use super::{SerializeBareItem, ValidateValue};
use crate::{utils, SFVResult};
use std::{convert::TryFrom, fmt, ops::Deref};

/// Tokens are short textual words; their abstract model is identical to their expression in the HTTP field value serialization.
///
/// The ABNF for Tokens is:
/// ```abnf,ignore,no_run
/// sf-token = ( ALPHA / "*" ) *( tchar / ":" / "/" )
/// ```
#[derive(Debug, PartialEq, Clone)]
pub struct BareItemToken(pub(crate) String);

impl Deref for BareItemToken {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl TryFrom<String> for BareItemToken {
    type Error = &'static str;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        let value = Self::validate(&value)?;
        Ok(BareItemToken(value.to_owned()))
    }
}

impl TryFrom<&str> for BareItemToken {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let value = Self::validate(value)?;
        Ok(BareItemToken(value.to_owned()))
    }
}

impl<'a> ValidateValue<'a, &'a str> for BareItemToken {
    fn validate(value: &'a str) -> SFVResult<&'a str> {
        if !value.is_ascii() {
            return Err("serialize_string: non-ascii character");
        }

        let mut chars = value.chars();
        if let Some(char) = chars.next() {
            if !(char.is_ascii_alphabetic() || char == '*') {
                return Err("serialise_token: first character is not ALPHA or '*'");
            }
        }

        if chars
            .clone()
            .any(|c| !(utils::is_tchar(c) || c == ':' || c == '/'))
        {
            return Err("serialise_token: disallowed character");
        }

        Ok(value)
    }
}

impl fmt::Display for BareItemToken {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl SerializeBareItem<&str> for BareItemToken {
    fn serialize_ref(value: &str, output: &mut String) {
        // https://httpwg.org/specs/rfc8941.html#ser-token
        output.push_str(value);
    }
}
