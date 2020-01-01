//! Telegram Bot API methods.

pub use telegram_bot_async_raw::{
    AnswerCallbackQuery, CanExportChatInviteLink, DeleteMessage, EditMessageCaption,
    EditMessageLiveLocation, EditMessageReplyMarkup, EditMessageText, ExportChatInviteLink,
    ForwardMessage, GetChat, GetChatAdministrators, GetChatMember, GetChatMembersCount, GetFile,
    GetMe, GetUpdates, GetUserProfilePhotos, KickChatMember, LeaveChat, PinChatMessage,
    RestrictChatMember, SendAudio, SendChatAction, SendContact, SendLocation, SendMessage,
    SendVenue, StopMessageLiveLocation, UnbanChatMember, UnpinChatMessage,
};
