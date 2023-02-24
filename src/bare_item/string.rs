use super::{SerializeBareItem, ValidateValue};
use crate::SFVResult;
use std::{convert::TryFrom, fmt, ops::Deref};

/// Strings are zero or more printable ASCII (RFC0020) characters (i.e., the range %x20 to %x7E). Note that this excludes tabs, newlines, carriage returns, etc.
///
/// The ABNF for Strings is:
/// ```abnf,ignore,no_run
/// sf-string = DQUOTE *chr DQUOTE
/// chr       = unescaped / escaped
/// unescaped = %x20-21 / %x23-5B / %x5D-7E
/// escaped   = "\" ( DQUOTE / "\" )
/// ```
#[derive(Debug, PartialEq, Clone)]
pub struct BareItemString(pub(crate) std::string::String);

impl Deref for BareItemString {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl TryFrom<String> for BareItemString {
    type Error = &'static str;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        let value = Self::validate(&value)?;
        Ok(BareItemString(value.to_owned()))
    }
}

impl TryFrom<&str> for BareItemString {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let value = Self::validate(value)?;
        Ok(BareItemString(value.to_owned()))
    }
}

impl<'a> ValidateValue<'a, &'a str> for BareItemString {
    fn validate(value: &'a str) -> SFVResult<&'a str> {
        if !value.is_ascii() {
            return Err("serialize_string: non-ascii character");
        }

        let vchar_or_sp = |char| char == '\x7f' || ('\x00'..='\x1f').contains(&char);
        if value.chars().any(vchar_or_sp) {
            return Err("serialize_string: not a visible character");
        }

        Ok(value)
    }
}

impl fmt::Display for BareItemString {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl SerializeBareItem<&str> for BareItemString {
    fn serialize_ref(value: &str, output: &mut String) {
        // https://httpwg.org/specs/rfc8941.html#ser-integer

        output.push('\"');
        for char in value.chars() {
            if char == '\\' || char == '\"' {
                output.push('\\');
            }
            output.push(char);
        }
        output.push('\"');
    }
}
