use crate::types::*;

/// Telegram bot module error type.
#[derive(Debug, Fail)]
pub enum Error {
    /// Empty body error.
    #[fail(display = "empty body")]
    EmptyBody,

    /// Telegram error.
    #[fail(display = "Telegram error: {} params: {:?}", _0, _1)]
    Telegram(String, Option<ResponseParameters>),

    /// Detached error.
    #[fail(display = "Detached error: {}", _0)]
    Detached(String),

    /// Serde JSON error.
    #[fail(display = "serde json error: {:?}", _0)]
    Json(#[cause] serde_json::Error),
}

impl From<serde_json::Error> for Error {
    fn from(err: serde_json::Error) -> Error {
        Error::Json(err)
    }
}
