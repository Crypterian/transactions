use rust_decimal::prelude::*;
use rust_decimal_macros::dec;
use serde::Serialize;
use serde::Serializer;

#[derive(Serialize, Debug)]
pub struct Account {
    pub(crate) client: u16,
    #[serde(serialize_with = "serialize_decimal_from_str")]
    pub(crate) available: Decimal,
    #[serde(serialize_with = "serialize_decimal_from_str")]
    pub(crate) held: Decimal,
    #[serde(serialize_with = "serialize_decimal_from_str")]
    pub(crate) total: Decimal,
    pub(crate) locked: bool,
}

pub fn serialize_decimal_from_str<S>(f: &Decimal, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    //Round to 4 decimals
    let s = f
        .round_dp_with_strategy(4, RoundingStrategy::MidpointAwayFromZero)
        .to_string();
    serializer.serialize_str(&s)
}

impl Account {
    pub(crate) fn new(client: u16) -> Self {
        Account {
            client,
            available: dec!(0.0),
            held: dec!(0.0),
            total: dec!(0.0),
            locked: false,
        }
    }
}
