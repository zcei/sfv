use super::{SerializeBareItem, ValidateValue};
use crate::SFVResult;
use rust_decimal::prelude::ToPrimitive;
use std::{convert::TryFrom, fmt, ops::Deref};

/// Decimals are numbers with an integer and a fractional component. The integer component has at most 12 digits; the fractional component has at most three digits.
///
/// The ABNF for decimals is:
/// ```abnf,ignore,no_run
/// sf-decimal  = ["-"] 1*12DIGIT "." 1*3DIGIT
/// ```
#[derive(Debug, PartialEq, Clone)]
pub struct BareItemDecimal(pub(crate) rust_decimal::Decimal);

impl TryFrom<rust_decimal::Decimal> for BareItemDecimal {
    type Error = &'static str;
    fn try_from(value: rust_decimal::Decimal) -> Result<Self, Self::Error> {
        let validated = Self::validate(value)?;
        Ok(BareItemDecimal(validated))
    }
}

impl ValidateValue<'_, rust_decimal::Decimal> for BareItemDecimal {
    fn validate(value: rust_decimal::Decimal) -> SFVResult<rust_decimal::Decimal> {
        let fraction_length = 3;

        let decimal = value.round_dp(fraction_length);
        let int_comp = decimal.trunc();
        let int_comp = int_comp
            .abs()
            .to_u64()
            .ok_or("serialize_decimal: integer component > 12 digits")?;

        if int_comp > 999_999_999_999_u64 {
            return Err("serialize_decimal: integer component > 12 digits");
        }

        Ok(decimal)
    }
}

impl Deref for BareItemDecimal {
    type Target = rust_decimal::Decimal;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl fmt::Display for BareItemDecimal {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl SerializeBareItem<&rust_decimal::Decimal> for BareItemDecimal {
    fn serialize_ref(value: &rust_decimal::Decimal, output: &mut String) {
        // https://httpwg.org/specs/rfc8941.html#ser-decimal
        let decimal = value;

        if decimal.fract().is_zero() {
            output.push_str(&format!("{:.1}", &decimal));
        } else {
            output.push_str(&decimal.to_string());
        }
    }
}
