use std::fmt::Debug;

use async_trait::async_trait;

use telegram_bot_async_raw::{HttpRequest, HttpResponse};

use crate::Error;

/// Connector provides basic IO with Telegram Bot API server.
#[async_trait]
pub trait Connector: Clone + Debug + Send + Sync + 'static {
    async fn request(
        &self,
        url: Option<&str>,
        token: &str,
        req: HttpRequest,
    ) -> Result<HttpResponse, Error>;
}
