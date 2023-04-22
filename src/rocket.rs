use crate::Decimal;
use core::str::FromStr;
use rocket::form::{self, FromFormField, ValueField};

impl<'v> FromFormField<'v> for Decimal {
    fn default() -> Option<Self> {
        None
    }
    fn from_value(field: ValueField<'v>) -> form::Result<'v, Self> {
        Decimal::from_str(field.value).map_err(|_| form::Error::validation("not a valid number").into())
    }
}
