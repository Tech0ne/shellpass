use crate::errors_builder::errors;

use std::fmt::Display;

use paste::paste;

errors!(
    AesGcm,
    Arboard,
    Argon2,
    Base64,
    Clipboard,
    Crossterm,
    Hash,
    Mkdir,
    Read,
    CiboriumEncode,
    CiboriumDecode,
    Write,
);

impl From<arboard::Error> for Error {
    fn from(value: arboard::Error) -> Self {
        Self::arboard(value)
    }
}

impl From<argon2::password_hash::Error> for Error {
    fn from(value: argon2::password_hash::Error) -> Self {
        Self::argon2(value)
    }
}

impl From<aes_gcm::Error> for Error {
    fn from(value: aes_gcm::Error) -> Self {
        Self::aes_gcm(value)
    }
}

impl From<base64::DecodeError> for Error {
    fn from(value: base64::DecodeError) -> Self {
        Self::base64(value)
    }
}

impl From<ciborium::ser::Error<std::io::Error>> for Error {
    fn from(value: ciborium::ser::Error<std::io::Error>) -> Self {
        Self::ciborium_encode(value)
    }
}

impl From<ciborium::de::Error<std::io::Error>> for Error {
    fn from(value: ciborium::de::Error<std::io::Error>) -> Self {
        Self::ciborium_decode(value)
    }
}

pub type Result<T> = core::result::Result<T, Error>;
