//! Telegram Bot API methods.

pub use telegram_bot_fork_raw::AnswerCallbackQuery;
pub use telegram_bot_fork_raw::SendChatAction;
pub use telegram_bot_fork_raw::{DeleteMessage, ForwardMessage};
pub use telegram_bot_fork_raw::{EditMessageCaption, EditMessageReplyMarkup, EditMessageText};
pub use telegram_bot_fork_raw::{EditMessageLiveLocation, StopMessageLiveLocation};
pub use telegram_bot_fork_raw::{GetChat, LeaveChat};
pub use telegram_bot_fork_raw::{GetChatAdministrators, GetChatMember, GetChatMembersCount};
pub use telegram_bot_fork_raw::{GetFile, GetMe, GetUpdates, GetUserProfilePhotos};
pub use telegram_bot_fork_raw::{KickChatMember, RestrictChatMember, UnbanChatMember};
pub use telegram_bot_fork_raw::{PinChatMessage, UnpinChatMessage};
pub use telegram_bot_fork_raw::{SendAudio, SendContact, SendLocation, SendMessage, SendVenue};
