//! Telegram bot types.

pub mod requests;
pub use requests::*;

pub use telegram_bot_fork_raw::{AllowedUpdate, Update, UpdateKind};
pub use telegram_bot_fork_raw::{Audio, Document, PhotoSize, Sticker, Video, Voice};
pub use telegram_bot_fork_raw::{CallbackQuery, CallbackQueryId};
pub use telegram_bot_fork_raw::{Channel, Chat, Group, MessageChat, Supergroup, User};
pub use telegram_bot_fork_raw::{ChannelId, ChatId, ChatRef, GroupId, SupergroupId, UserId};
pub use telegram_bot_fork_raw::{ChannelPost, Message, MessageOrChannelPost};
pub use telegram_bot_fork_raw::{Contact, File, FileRef, Location, Venue};
pub use telegram_bot_fork_raw::{DetachedRequest, Float, Integer, Request};
pub use telegram_bot_fork_raw::{Forward, ForwardFrom, MessageId, MessageKind};
pub use telegram_bot_fork_raw::{
    JsonIdResponse, JsonResponse, JsonTrueToUnitResponse, ResponseType,
};
pub use telegram_bot_fork_raw::{MessageEntity, MessageEntityKind};
pub use telegram_bot_fork_raw::{
    ToCallbackQueryId, ToChatRef, ToFileRef, ToMessageId, ToSourceChat, ToUserId,
};

pub use telegram_bot_fork_raw::ChatAction;
pub use telegram_bot_fork_raw::ParseMode;
pub use telegram_bot_fork_raw::{ChatMember, ChatMemberStatus};
pub use telegram_bot_fork_raw::{ForceReply, ReplyKeyboardRemove};
pub use telegram_bot_fork_raw::{InlineKeyboardButton, InlineKeyboardMarkup};
pub use telegram_bot_fork_raw::{KeyboardButton, ReplyKeyboardMarkup, ReplyMarkup};
