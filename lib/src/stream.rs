use std::{cmp::max, time::Duration};

use futures_async_stream::async_try_stream;
use telegram_bot_fork_raw::{AllowedUpdate, GetUpdates, Integer, Update};

use crate::{api::Api, errors::Error, Connector};

const TELEGRAM_LONG_POLL_TIMEOUT_SECONDS: u64 = 5;
const TELEGRAM_LONG_POLL_LIMIT_MESSAGES: Integer = 100;

/// This type represents stream of Telegram API updates and uses
/// long polling method under the hood.
#[must_use = "streams do nothing unless polled"]
pub struct UpdatesStream<C: Connector> {
    api: Api<C>,
    last_update: Integer,
    timeout: Duration,
    allowed_updates: Vec<AllowedUpdate>,
    limit: Integer,
}

impl<C: Connector> UpdatesStream<C> {
    /// Creates a stream of updates.
    #[async_try_stream(ok = Update, error = Error)]
    pub async fn updates(&mut self) {
        let request = GetUpdates::new()
            .offset(self.last_update + 1)
            .timeout(self.timeout.as_secs() as Integer)
            .allowed_updates(&self.allowed_updates)
            .limit(self.limit);
        let updates = self.api.send_timeout(request, self.timeout).await?;

        for update in updates {
            self.last_update = max(update.id, self.last_update);
            yield update;
        }
    }
}

pub trait NewUpdatesStream<C: Connector> {
    fn new(api: Api<C>) -> Self;
}

impl<C: Connector> NewUpdatesStream<C> for UpdatesStream<C> {
    fn new(api: Api<C>) -> Self {
        UpdatesStream {
            api,
            last_update: 0,
            timeout: Duration::from_secs(TELEGRAM_LONG_POLL_TIMEOUT_SECONDS),
            allowed_updates: Vec::new(),
            limit: TELEGRAM_LONG_POLL_LIMIT_MESSAGES,
        }
    }
}
