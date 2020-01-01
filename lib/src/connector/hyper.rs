//! Connector with hyper backend.

use async_trait::async_trait;
use hyper::{
    self,
    body::Buf,
    client::{connect::Connect, Client, HttpConnector},
    Body, Method, Request,
};
use hyper_tls::HttpsConnector;

use telegram_bot_async_raw::{
    Body as TelegramBody, HttpRequest, HttpResponse, Method as TelegramMethod,
};

use crate::errors::Error;

use super::Connector;

/// Default Hyper connector type.
pub type DefaultConnector = HyperConnector<HttpsConnector<HttpConnector>>;

/// This connector uses `hyper` backend.
#[derive(Clone)]
pub struct HyperConnector<C> {
    inner: Client<C>,
}

impl<C> std::fmt::Debug for HyperConnector<C> {
    fn fmt(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        "hyper connector".fmt(formatter)
    }
}

impl<C> HyperConnector<C> {
    pub fn new(client: Client<C>) -> Self {
        HyperConnector { inner: client }
    }
}

#[async_trait]
impl<C: Connect + Send + Sync + Clone + 'static> Connector for HyperConnector<C> {
    async fn request(
        &self,
        url: Option<&str>,
        token: &str,
        req: HttpRequest,
    ) -> Result<HttpResponse, Error> {
        let method = match req.method {
            TelegramMethod::Get => Method::GET,
            TelegramMethod::Post => Method::POST,
        };
        let body = match req.body {
            TelegramBody::Empty => Body::empty(),
            TelegramBody::Json(body) => body.into(),
        };
        let request = Request::builder()
            .method(method)
            .uri(&req.url.url(url, token))
            .header(hyper::header::CONTENT_TYPE, "application/json")
            .body(body)?;
        let response = self.inner.request(request).await?;
        let body = hyper::body::aggregate(response).await?;
        Ok(HttpResponse {
            body: Some(body.bytes().to_vec()),
        })
    }
}

/// Returns default hyper connector. Uses one resolve thread and `HttpsConnector`.
pub fn default_connector() -> Result<DefaultConnector, Error> {
    let https = HttpsConnector::new();
    let client = hyper::Client::builder().build::<_, hyper::Body>(https);
    Ok(HyperConnector::new(client))
}
