use std::{fmt, ops::Deref};

use super::SerializeBareItem;

/// Boolean values can be conveyed in Structured Fields.
///
/// The ABNF for a Boolean is:
/// ```abnf,ignore,no_run
/// sf-boolean = "?" boolean
/// boolean    = "0" / "1"
/// ```
#[derive(Debug, PartialEq, Clone)]
pub struct BareItemBoolean(pub(crate) bool);

impl From<bool> for BareItemBoolean {
    fn from(value: bool) -> Self {
        BareItemBoolean(value)
    }
}

impl Deref for BareItemBoolean {
    type Target = bool;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl fmt::Display for BareItemBoolean {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl SerializeBareItem<bool> for BareItemBoolean {
    fn serialize_ref(value: bool, output: &mut String) {
        // https://httpwg.org/specs/rfc8941.html#ser-boolean

        let val = if value { "?1" } else { "?0" };
        output.push_str(val);
    }
}
