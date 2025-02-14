use super::error::{Result, TypeError};
use api_types::api::{generic::ApiCryptoCurrency, viviswap::detail::SwapPaymentDetailKey};
use iota_sdk::client::constants::{ETHER_COIN_TYPE, IOTA_COIN_TYPE};
use serde::Serialize;

/// Supported currencies (mirrors `api_types` but needed so we can implement the additional
/// `coin_type` and `to_vivi_payment_method_key` function)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize)]
pub enum Currency {
    /// Iota token
    Iota,
    /// Ethereum token
    Eth,
}

impl TryFrom<String> for Currency {
    type Error = TypeError;
    /// Convert from String to Currency, used at the API boundary to interface with the bindings.
    fn try_from(currency: String) -> Result<Self> {
        match currency.to_lowercase().as_str() {
            "iota" => Ok(Self::Iota),
            "eth" => Ok(Self::Eth),
            _ => Err(TypeError::InvalidCurrency(currency)),
        }
    }
}

impl Currency {
    /// Returns the coin type for the [`Currency`] according to [`SLIP-044`].
    ///
    /// [`SLIP-044`]: https://github.com/satoshilabs/slips/blob/master/slip-0044.md
    pub fn coin_type(self) -> u32 {
        match self {
            Self::Iota => IOTA_COIN_TYPE,
            Self::Eth => ETHER_COIN_TYPE,
        }
    }

    /// Convert this [`Currency`] into a [`SwapPaymentDetailKey`]
    pub fn to_vivi_payment_method_key(self) -> SwapPaymentDetailKey {
        match self {
            Self::Iota => SwapPaymentDetailKey::Iota,
            Self::Eth => SwapPaymentDetailKey::Eth,
        }
    }
}

/// We want to convert from our internal Currency enum into the one used in the API.
impl From<Currency> for ApiCryptoCurrency {
    fn from(value: Currency) -> Self {
        match value {
            Currency::Iota => ApiCryptoCurrency::Iota,
            Currency::Eth => ApiCryptoCurrency::Eth,
        }
    }
}
impl From<ApiCryptoCurrency> for Currency {
    fn from(value: ApiCryptoCurrency) -> Self {
        match value {
            ApiCryptoCurrency::Iota => Currency::Iota,
            ApiCryptoCurrency::Eth => Currency::Eth,
        }
    }
}

// the display implementation must be compatible with TryFrom<String> since it is part of the
// public binding interface.
impl std::fmt::Display for Currency {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Currency::Iota => write!(f, "Iota"),
            Currency::Eth => write!(f, "Eth"),
        }
    }
}

/// A non-negative decimal value. Used as inputs to create purchases or sending a transaction.
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct CryptoAmount(rust_decimal::Decimal);

impl CryptoAmount {
    /// The value of ZERO
    pub const ZERO: Self = Self(rust_decimal::Decimal::ZERO);

    /// Get the inner value of the amount
    pub fn inner(&self) -> rust_decimal::Decimal {
        self.0
    }

    /// Internal helper function to create values in consts during tests.
    /// This is unsafe since it does not perform any non-negativity checks.
    pub(crate) const unsafe fn new_unchecked(value: rust_decimal::Decimal) -> Self {
        Self(value)
    }
}

// From u64 is always possible and will yield a Non-negative value
impl From<u64> for CryptoAmount {
    fn from(value: u64) -> Self {
        Self(rust_decimal::Decimal::from(value))
    }
}

impl TryFrom<f64> for CryptoAmount {
    type Error = crate::Error;

    fn try_from(value: f64) -> std::result::Result<Self, Self::Error> {
        Self::try_from(rust_decimal::Decimal::try_from(value)?)
    }
}

impl TryFrom<rust_decimal::Decimal> for CryptoAmount {
    type Error = crate::Error;

    fn try_from(value: rust_decimal::Decimal) -> std::result::Result<Self, Self::Error> {
        if value < rust_decimal::Decimal::ZERO {
            return Err(crate::Error::NegativeAmount);
        }
        Ok(Self(value))
    }
}

impl TryFrom<CryptoAmount> for f64 {
    type Error = crate::Error;

    fn try_from(value: CryptoAmount) -> std::result::Result<Self, Self::Error> {
        Ok(value.0.try_into()?)
    }
}

// Adding two NonNegativeAmounts will always result in a positive value so this is safe
impl std::ops::Add for CryptoAmount {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0)
    }
}

// Dividing two NonNegativeAmounts will always result in a positive value so this is safe
impl std::ops::Div for CryptoAmount {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        Self(self.0 / rhs.0)
    }
}

// Multiplying two NonNegativeAmounts will always result in a positive value so this is safe
impl std::ops::Mul for CryptoAmount {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self(self.0 * rhs.0)
    }
}

#[cfg(test)]
mod test {
    use crate::types::currencies::Currency;

    use rust_decimal_macros::dec;

    use super::CryptoAmount;

    #[rstest::rstest]
    fn test_display_roundtrip(#[values(Currency::Iota, Currency::Eth)] c: Currency) {
        assert_eq!(c, Currency::try_from(c.to_string()).unwrap());
    }

    #[rstest::rstest]
    #[case(dec!(0.0), Some(CryptoAmount(dec!(0.0))))]
    #[case(dec!(-0.0), Some(CryptoAmount(dec!(-0.0))))]
    #[case(dec!(-1.0), None)]
    #[case(dec!(-10.5), None)]
    #[case(dec!(1.0), Some(CryptoAmount(dec!(1.0))))]
    #[case(dec!(10.0), Some(CryptoAmount(dec!(10.0))))]
    fn test_try_from_non_zero_dec(#[case] value: rust_decimal::Decimal, #[case] expected_value: Option<CryptoAmount>) {
        let amount = CryptoAmount::try_from(value);

        match (amount, expected_value) {
            (Ok(amount), Some(expected)) => assert_eq!(amount, expected),
            (Err(error), None) => assert!(matches!(error, crate::Error::NegativeAmount)),
            (amount, expected) => panic!("expected {expected:?} but got {amount:?} for {value}"),
        }
    }

    #[rstest::rstest]
    #[case(0.0, Some(CryptoAmount(dec!(0.0))))]
    #[case(-0.0, Some(CryptoAmount(dec!(0.0))))] // this is apparently also "negative"
    #[case(-1.0, None)]
    #[case(-10.5, None)]
    #[case(1.0, Some(CryptoAmount(dec!(1.0))))]
    #[case(10.0, Some(CryptoAmount(dec!(10.0))))]
    fn test_try_from_non_zero_f64(#[case] value: f64, #[case] expected_value: Option<CryptoAmount>) {
        let amount = CryptoAmount::try_from(value);

        match (amount, expected_value) {
            (Ok(amount), Some(expected)) => assert_eq!(amount, expected),
            (Err(error), None) => assert!(matches!(error, crate::Error::NegativeAmount)),
            (amount, expected) => panic!("expected {expected:?} but got {amount:?} for {value}"),
        }
    }
}
