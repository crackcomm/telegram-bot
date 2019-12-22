use std::time::Duration;

use telegram_bot_fork_raw::{Request, ResponseType};

use crate::connector::Connector;
#[cfg(feature = "hyper_connector")]
use crate::{
    connector::{default_connector, hyper::DefaultConnector},
    errors::Error,
};
// stream::{NewUpdatesStream, UpdatesStream},

/// Default API client type.
pub type DefaultApi = Api<DefaultConnector>;

/// Main type for sending requests to the Telegram bot API.
#[derive(Clone)]
pub struct Api<C: Connector> {
    url: Option<String>,
    token: String,
    connector: C,
}

impl<C: Connector> Api<C> {
    /// Start construction of the `Api` instance.
    ///
    /// # Examples
    ///
    /// Using default connector.
    ///
    /// ```rust
    /// # extern crate telegram_bot_fork;
    /// # extern crate tokio;
    /// use telegram_bot_fork::DefaultApi;
    ///
    /// # fn main() {
    /// # let telegram_token = "token";
    /// let api = DefaultApi::new_default(telegram_token.to_string()).unwrap();
    /// # }
    /// ```
    #[cfg(feature = "hyper_connector")]
    pub fn new_default(token: String) -> Result<DefaultApi, Error> {
        Ok(Api::with_connector(token, default_connector()?))
    }

    /// Creates new API using custom connector.
    ///
    ///
    /// ```rust
    /// # extern crate telegram_bot_fork;
    /// # extern crate tokio;
    /// # #[cfg(feature = "hyper_connector")]
    /// # fn main() {
    /// use telegram_bot_fork::{connector::hyper, Api};
    ///
    /// # let telegram_token = "token";
    /// let api = Api::with_connector(telegram_token.to_string(), hyper::default_connector().unwrap());
    /// # }
    ///
    /// # #[cfg(not(feature = "hyper_connector"))]
    /// # fn main() {}
    /// ```
    pub fn with_connector(token: String, connector: C) -> Api<C> {
        Api {
            url: None,
            token,
            connector,
        }
    }

    /// Sets base telegram API server URL.
    pub fn set_url<T: AsRef<str>>(&mut self, url: T) -> &mut Self {
        self.url = Some(url.as_ref().into());
        self
    }

    // /// Create a stream which produces updates from the Telegram server.
    // ///
    // /// # Examples
    // ///
    // /// ```rust
    // /// # extern crate futures;
    // /// # extern crate telegram_bot_fork;
    // /// # extern crate tokio;
    // /// # use telegram_bot_fork::DefaultApi;
    // ///
    // /// # #[tokio::main]
    // /// # pub async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    // /// # let api = DefaultApi::new_default("token".to_string()).unwrap();
    // /// use futures::Stream;
    // ///
    // /// let future = api.stream().for_each(|update| {
    // ///     println!("{:?}", update);
    // ///     Ok(())
    // /// });
    // /// Ok(())
    // /// # }
    // /// ```
    // pub fn stream(&self) -> UpdatesStream {
    //     UpdatesStream::new(self.clone())
    // }

    /// Send a request to the Telegram server and wait for a response, timing out after `duration`.
    /// Future will resolve to `None` if timeout fired.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # extern crate futures;
    /// # extern crate telegram_bot_fork;
    /// # extern crate tokio;
    /// # use futures::Future;
    /// # use telegram_bot_fork::{DefaultApi, GetMe, Error};
    /// #
    /// # #[tokio::main]
    /// # pub async fn main() -> Result<(), Error> {
    /// # let telegram_token = "token";
    /// # let api = DefaultApi::new_default(telegram_token.to_string()).unwrap();
    /// # if false {
    /// use std::time::Duration;
    ///
    /// let resp = api.send_timeout(GetMe, Duration::from_secs(5)).await?;
    /// # }
    /// Ok(())
    /// # }
    /// ```
    pub async fn send_timeout<Req: Request>(
        &self,
        request: Req,
        duration: Duration,
    ) -> Result<<Req::Response as ResponseType>::Type, Error> {
        tokio::time::timeout(duration, self.send(request)).await?
    }

    /// Send a request to the Telegram server and wait for a response.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # extern crate futures;
    /// # extern crate telegram_bot_fork;
    /// # extern crate tokio;
    /// # use futures::Future;
    /// # use telegram_bot_fork::{DefaultApi, GetMe, Error};
    /// #
    /// # #[tokio::main]
    /// # pub async fn main() -> Result<(), Error> {
    /// # let telegram_token = "token";
    /// # let api = DefaultApi::new_default(telegram_token.to_string()).unwrap();
    /// # if false {
    /// let resp = api.send(GetMe).await?;
    /// # }
    /// Ok(())
    /// # }
    /// ```
    pub async fn send<Req: Request>(
        &self,
        request: Req,
    ) -> Result<<Req::Response as ResponseType>::Type, Error> {
        let request = request.serialize()?;

        let response = self
            .connector
            .request(self.url.as_ref().map(String::as_str), &self.token, request)
            .await?;

        Ok(Req::Response::deserialize(response)?)
    }
}
