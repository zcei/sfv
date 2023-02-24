use super::{SerializeBareItem, ValidateValue};
use crate::SFVResult;
use std::{convert::TryFrom, fmt, ops::Deref};

/// Integers have a range of -999,999,999,999,999 to 999,999,999,999,999 inclusive (i.e., up to fifteen digits, signed), for IEEE 754 compatibility.
///
/// The ABNF for Integers is:
/// ```abnf,ignore,no_run
/// sf-integer = ["-"] 1*15DIGIT
/// ```
#[derive(Debug, PartialEq, Clone)]
pub struct BareItemInteger(pub(crate) i64);

impl Deref for BareItemInteger {
    type Target = i64;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl TryFrom<i64> for BareItemInteger {
    type Error = &'static str;

    fn try_from(value: i64) -> Result<Self, Self::Error> {
        let value = Self::validate(value)?;
        Ok(BareItemInteger(value))
    }
}

impl ValidateValue<'_, i64> for BareItemInteger {
    fn validate(value: i64) -> SFVResult<i64> {
        let (min_int, max_int) = (-999_999_999_999_999_i64, 999_999_999_999_999_i64);

        if !(min_int <= value && value <= max_int) {
            return Err("serialize_integer: integer is out of range");
        }

        Ok(value)
    }
}

impl fmt::Display for BareItemInteger {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl SerializeBareItem<&i64> for BareItemInteger {
    fn serialize_ref(value: &i64, output: &mut String) {
        // https://httpwg.org/specs/rfc8941.html#ser-integer
        output.push_str(&value.to_string());
    }
}
