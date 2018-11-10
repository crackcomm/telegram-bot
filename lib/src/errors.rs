use telegram_bot_fork_raw;

error_chain! {
    foreign_links {
        Url(::http::uri::InvalidUri) #[cfg(feature = "hyper_connector")];
        Hyper(::hyper::Error) #[cfg(feature = "hyper_connector")];
        Io(::std::io::Error);
        Tokio(::tokio_timer::Error);
    }

    links {
        Raw(telegram_bot_fork_raw::Error, telegram_bot_fork_raw::ErrorKind);
    }
}
