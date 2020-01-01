use telegram_bot_async_raw;

/// Telegram bot module error type.
#[derive(Debug, Fail)]
pub enum Error {
    /// Option is `None` error.
    #[fail(display = "option none")]
    OptionNone,

    /// Telegram Raw API error.
    #[fail(display = "api error: {}", _0)]
    Api(#[cause] telegram_bot_async_raw::Error),

    /// Timeout error.
    #[fail(display = "timeout error: {}", _0)]
    Timeout(#[cause] tokio::time::Elapsed),

    /// IO error.
    #[fail(display = "IO error: {}", _0)]
    Io(#[cause] std::io::Error),

    /// Hyper error.
    #[cfg(feature = "hyper_connector")]
    #[fail(display = "hyper error: {:?}", _0)]
    Hyper(::hyper::Error),

    /// Hyper HTTP error.
    #[cfg(feature = "hyper_connector")]
    #[fail(display = "hyper http error: {:?}", _0)]
    HyperHttp(::hyper::http::Error),
}

#[macro_export]
macro_rules! err_converter {
    ($a:ident, $b:ty) => {
        impl From<$b> for Error {
            fn from(e: $b) -> Self {
                Error::$a(e)
            }
        }
    };
}

err_converter!(Io, std::io::Error);
err_converter!(Api, telegram_bot_async_raw::Error);
err_converter!(Timeout, tokio::time::Elapsed);
#[cfg(feature = "hyper_connector")]
err_converter!(Hyper, ::hyper::Error);
#[cfg(feature = "hyper_connector")]
err_converter!(HyperHttp, ::hyper::http::Error);

impl From<std::option::NoneError> for Error {
    fn from(_: std::option::NoneError) -> Error {
        Error::OptionNone
    }
}
