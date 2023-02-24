mod boolean;
mod byte_sequence;
mod decimal;
mod integer;
mod string;
mod token;

use crate::SFVResult;
use rust_decimal::prelude::FromPrimitive;
use std::{
    convert::{TryFrom, TryInto},
    fmt::Debug,
};

pub use self::boolean::BareItemBoolean;
pub use self::byte_sequence::BareItemByteSeq;
pub use self::decimal::BareItemDecimal;
pub use self::integer::BareItemInteger;
pub use self::string::BareItemString;
pub use self::token::BareItemToken;

/// `BareItem` type is used to construct `Items` or `Parameters` values.
#[derive(Debug, PartialEq, Clone)]
pub enum BareItem {
    /// Decimal number
    // sf-decimal  = ["-"] 1*12DIGIT "." 1*3DIGIT
    Decimal(BareItemDecimal),
    /// Integer number
    // sf-integer = ["-"] 1*15DIGIT
    Integer(BareItemInteger),
    // sf-string = DQUOTE *chr DQUOTE
    // chr       = unescaped / escaped
    // unescaped = %x20-21 / %x23-5B / %x5D-7E
    // escaped   = "\" ( DQUOTE / "\" )
    String(BareItemString),
    // ":" *(base64) ":"
    // base64    = ALPHA / DIGIT / "+" / "/" / "="
    ByteSeq(BareItemByteSeq),
    // sf-boolean = "?" boolean
    // boolean    = "0" / "1"
    Boolean(BareItemBoolean),
    // sf-token = ( ALPHA / "*" ) *( tchar / ":" / "/" )
    Token(BareItemToken),
}

impl BareItem {
    /// Creates a `BareItem::Decimal` from an `f64` input.
    /// ```
    /// # use sfv::BareItem;
    /// # fn main() -> Result<(), &'static str> {
    /// let value = BareItem::new_decimal_from_f64(13.37)?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn new_decimal_from_f64(value: f64) -> SFVResult<BareItem> {
        let decimal = rust_decimal::Decimal::from_f64(value)
            .ok_or("validate_decimal: value can not represent decimal")?;

        Self::new_decimal(decimal)
    }

    /// Creates a `BareItem::Decimal` from a `rust_decimal::Decimal` input.
    /// ```
    /// # use sfv::BareItem;
    /// # use crate::sfv::FromPrimitive;
    /// # fn main() -> Result<(), &'static str> {
    /// let decimal = rust_decimal::Decimal::from_f64(13.37).unwrap();
    /// let value = BareItem::new_decimal(decimal);
    /// # Ok(())
    /// # }
    /// ```
    pub fn new_decimal(value: rust_decimal::Decimal) -> SFVResult<BareItem> {
        let value: BareItemDecimal = value.try_into()?;
        Ok(BareItem::Decimal(value))
    }

    /// Creates a `BareItem::Integer` from a `i64` input.
    /// ```
    /// # use sfv::BareItem;
    /// # fn main() -> Result<(), &'static str> {
    /// let value = BareItem::new_integer(42)?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn new_integer(value: i64) -> SFVResult<BareItem> {
        let value: BareItemInteger = value.try_into()?;
        Ok(BareItem::Integer(value))
    }

    /// Creates a `BareItem::String` from a `&str` input.
    /// ```
    /// # use sfv::BareItem;
    /// # fn main() -> Result<(), &'static str> {
    /// let value = BareItem::new_string("foo")?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn new_string(value: &str) -> SFVResult<BareItem> {
        let value: BareItemString = value.try_into()?;
        Ok(BareItem::String(value))
    }

    /// Creates a `BareItem::ByteSeq` from a byte slice input.
    /// ```
    /// # use sfv::BareItem;
    /// # fn main() -> Result<(), &'static str> {
    /// let value = BareItem::new_byte_seq("hello".as_bytes())?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn new_byte_seq(value: &[u8]) -> SFVResult<BareItem> {
        let value: BareItemByteSeq = value.into();
        Ok(BareItem::ByteSeq(value))
    }

    /// Creates a `BareItem::Boolean` from a `bool` input.
    /// ```
    /// # use sfv::BareItem;
    /// # fn main() -> Result<(), &'static str> {
    /// let value = BareItem::new_boolean(true)?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn new_boolean(value: bool) -> SFVResult<BareItem> {
        let value: BareItemBoolean = value.into();
        Ok(BareItem::Boolean(value))
    }

    /// Creates a `BareItem::Token` from a `&str` input.
    /// ```
    /// # use sfv::BareItem;
    /// # fn main() -> Result<(), &'static str> {
    /// let value = BareItem::new_boolean(true)?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn new_token(value: &str) -> SFVResult<BareItem> {
        let value: BareItemToken = value.try_into()?;
        Ok(BareItem::Token(value))
    }
}

impl BareItem {
    /// If `BareItem` is a decimal, returns `Decimal`, otherwise returns `None`.
    /// ```
    /// # use sfv::{BareItem, FromPrimitive};
    /// use rust_decimal::Decimal;
    /// # use std::convert::TryInto;
    /// # fn main() -> Result<(), &'static str> {
    /// let decimal_number = Decimal::from_f64(415.566).unwrap();
    /// let bare_item: BareItem = decimal_number.try_into()?;
    /// assert_eq!(bare_item.as_decimal().unwrap(), decimal_number);
    /// # Ok(())
    /// # }
    /// ```
    pub fn as_decimal(&self) -> Option<rust_decimal::Decimal> {
        match self {
            BareItem::Decimal(val) => Some(val.0),
            _ => None,
        }
    }
    /// If `BareItem` is an integer, returns `i64`, otherwise returns `None`.
    /// ```
    /// # use sfv::BareItem;
    /// # use std::convert::TryInto;
    /// # fn main() -> Result<(), &'static str> {
    /// let bare_item: BareItem = 100_i64.try_into()?;
    /// assert_eq!(bare_item.as_int().unwrap(), 100);
    /// # Ok(())
    /// # }
    /// ```
    pub fn as_int(&self) -> Option<i64> {
        match &self {
            BareItem::Integer(val) => Some(**val),
            _ => None,
        }
    }
    /// If `BareItem` is `String`, returns `&str`, otherwise returns `None`.
    /// ```
    /// # use sfv::BareItem;
    /// # use std::convert::TryInto;
    /// # fn main() -> Result<(), &'static str> {
    /// let bare_item = BareItem::String("foo".to_owned().try_into()?);
    /// assert_eq!(bare_item.as_str().unwrap(), "foo");
    /// Ok(())
    /// # }
    /// ```
    pub fn as_str(&self) -> Option<&str> {
        match *self {
            BareItem::String(ref val) => Some(val),
            _ => None,
        }
    }
    /// If `BareItem` is a `ByteSeq`, returns `&Vec<u8>`, otherwise returns `None`.
    /// ```
    /// # use sfv::BareItem;
    /// let bare_item = BareItem::ByteSeq("foo".to_owned().into_bytes().into());
    /// assert_eq!(bare_item.as_byte_seq().unwrap().as_slice(), "foo".as_bytes());
    /// ```
    pub fn as_byte_seq(&self) -> Option<&Vec<u8>> {
        match *self {
            BareItem::ByteSeq(ref val) => Some(&val.0),
            _ => None,
        }
    }
    /// If `BareItem` is a `Boolean`, returns `bool`, otherwise returns `None`.
    /// ```
    /// # use sfv::{BareItem, FromPrimitive};
    /// let bare_item = BareItem::Boolean(true.into());
    /// assert_eq!(bare_item.as_bool().unwrap(), true);
    /// ```
    pub fn as_bool(&self) -> Option<bool> {
        match self {
            BareItem::Boolean(val) => Some(val.0),
            _ => None,
        }
    }
    /// If `BareItem` is a `Token`, returns `&str`, otherwise returns `None`.
    /// ```
    /// use sfv::BareItem;
    /// # use std::convert::TryInto;
    /// # fn main() -> Result<(), &'static str> {
    ///
    /// let bare_item = BareItem::Token("*bar".try_into()?);
    /// assert_eq!(bare_item.as_token().unwrap(), "*bar");
    /// # Ok(())
    /// # }
    /// ```
    pub fn as_token(&self) -> Option<&str> {
        match *self {
            BareItem::Token(ref val) => Some(val),
            _ => None,
        }
    }
}

impl BareItem {
    pub(crate) fn write(&self, output: &mut String) -> SFVResult<()> {
        match self {
            BareItem::Integer(val) => BareItemInteger::serialize_ref(val, output),
            BareItem::Decimal(val) => BareItemDecimal::serialize_ref(val, output),
            BareItem::String(val) => BareItemString::serialize_ref(val, output),
            BareItem::ByteSeq(val) => BareItemByteSeq::serialize_ref(val, output),
            BareItem::Boolean(val) => BareItemBoolean::serialize_ref(**val, output),
            BareItem::Token(val) => BareItemToken::serialize_ref(val, output),
        };

        Ok(())
    }
}

impl TryFrom<i64> for BareItem {
    type Error = &'static str;
    /// Converts `i64` into `BareItem::Integer`.
    /// ```
    /// # use sfv::BareItem;
    /// # use std::convert::TryInto;
    /// # fn main() -> Result<(), &'static str> {
    /// let bare_item: BareItem = 456_i64.try_into()?;
    /// assert_eq!(bare_item.as_int().unwrap(), 456);
    /// # Ok(())
    /// # }
    /// ```
    fn try_from(item: i64) -> Result<Self, Self::Error> {
        Self::new_integer(item)
    }
}

impl TryFrom<rust_decimal::Decimal> for BareItem {
    type Error = &'static str;
    /// Converts `rust_decimal::Decimal` into `BareItem::Decimal`.
    /// ```
    /// # use sfv::{BareItem, FromPrimitive};
    /// # use std::convert::TryInto;
    /// use rust_decimal::Decimal;
    /// # fn main() -> Result<(), &'static str> {
    /// let decimal_number = Decimal::from_f64(48.01).unwrap();
    /// let bare_item: BareItem = decimal_number.try_into()?;
    /// assert_eq!(bare_item.as_decimal().unwrap(), decimal_number);
    /// # Ok(())
    /// # }
    /// ```
    fn try_from(item: rust_decimal::Decimal) -> Result<Self, Self::Error> {
        Self::new_decimal(item)
    }
}

impl TryFrom<f64> for BareItem {
    type Error = &'static str;

    /// Converts `f64` into `BareItem::Decimal`.
    /// ```
    /// # use sfv::{BareItem, FromPrimitive};
    /// # use std::convert::TryInto;
    /// # use rust_decimal::prelude::ToPrimitive;
    /// # fn main() -> Result<(), &'static str> {
    /// let decimal_number = 48.01;
    /// let bare_item: BareItem = decimal_number.try_into()?;
    /// assert_eq!(bare_item.as_decimal().unwrap().to_f64().unwrap(), decimal_number);
    /// # Ok(())
    /// # }
    /// ```
    fn try_from(value: f64) -> Result<Self, Self::Error> {
        Self::new_decimal_from_f64(value)
    }
}

impl TryFrom<&[u8]> for BareItem {
    type Error = &'static str;

    /// Converts a byte slice into `BareItem::ByteSeq`.
    /// ```
    /// # use sfv::{BareItem, FromPrimitive};
    /// # use std::convert::TryInto;
    /// # fn main() -> Result<(), &'static str> {
    /// let byte_slice = "hello".as_bytes();
    /// let bare_item: BareItem = byte_slice.try_into()?;
    /// assert_eq!(bare_item.as_byte_seq().unwrap(), byte_slice);
    /// # Ok(())
    /// # }
    /// ```
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Self::new_byte_seq(value)
    }
}

impl TryFrom<bool> for BareItem {
    type Error = &'static str;

    /// Converts a `bool` into `BareItem::Boolean`.
    /// ```
    /// # use sfv::{BareItem, FromPrimitive};
    /// # use std::convert::TryInto;
    /// # fn main() -> Result<(), &'static str> {
    /// let boolean = true;
    /// let bare_item: BareItem = boolean.try_into()?;
    /// assert_eq!(bare_item.as_bool().unwrap(), boolean);
    /// # Ok(())
    /// # }
    /// ```
    fn try_from(value: bool) -> Result<Self, Self::Error> {
        Self::new_boolean(value)
    }
}

/// Validates a bare item value and returns a new sanitized value
/// or passes back ownership of the existing value in case the input needs no change.
pub trait ValidateValue<'a, T> {
    fn validate(value: T) -> SFVResult<T>;
}

pub trait SerializeBareItem<T> {
    fn serialize_ref(value: T, output: &mut String);
}

#[cfg(test)]
mod tests {
    use std::convert::TryInto;
    use std::error::Error;
    use std::str::FromStr;

    use super::*;

    #[test]
    fn create_non_ascii_string_errors() -> Result<(), Box<dyn Error>> {
        let disallowed_value: Result<BareItemString, &str> =
            "non-ascii text 🐹".to_owned().try_into();

        assert_eq!(
            Err("serialize_string: non-ascii character"),
            disallowed_value
        );

        Ok(())
    }

    #[test]
    fn create_too_long_decimal_errors() -> Result<(), Box<dyn Error>> {
        let disallowed_value: Result<BareItemDecimal, &str> =
            rust_decimal::Decimal::from_str("12345678912345.123")?.try_into();
        assert_eq!(
            Err("serialize_decimal: integer component > 12 digits"),
            disallowed_value
        );

        Ok(())
    }
}
