pub mod token;

use serde::{Serialize, Serializer};
use std::time::SystemTimeError;
use totp_rs::{Algorithm, TotpUrlError};

#[derive(Debug)]
pub struct Token {
    pub issuer: String,
    pub account_name: String,
    pub secret: String,
}

impl Token {
    pub fn new(issuer: String, account_name: String, secret: String) -> Self {
        Self {
            issuer,
            account_name,
            secret,
        }
    }
}

#[derive(Debug)]
pub struct Otp {
    pub algorithm: Algorithm,
    pub digits: usize,
    pub step: u64,
}

impl Otp {
    pub fn new(algorithm: Algorithm, digits: usize, step: u64) -> Self {
        Self {
            algorithm,
            digits,
            step,
        }
    }
}

#[derive(Debug)]
pub enum TotpError {
    UrlError(TotpUrlError),
    STError(SystemTimeError),
}

impl From<TotpUrlError> for TotpError {
    fn from(value: TotpUrlError) -> Self {
        Self::UrlError(value)
    }
}

impl From<SystemTimeError> for TotpError {
    fn from(value: SystemTimeError) -> Self {
        Self::STError(value)
    }
}

impl Serialize for TotpError {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match *self {
            TotpError::UrlError(ref err) => serializer.serialize_newtype_variant(
                "TotpError", 0, "UrlError", &err.to_string()
            ),
            TotpError::STError(ref err) => serializer.serialize_newtype_variant(
                "TotpError", 1, "STError", &err.to_string()
            ),
        }
    }
}
