use std::borrow::Cow;

#[derive(Serialize, Debug, Clone)]
pub struct GetUpdates{
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<i64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeout: Option<u32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub allowed_updates: Option<Vec<AllowedUpdate>>,
}

impl GetUpdates {
    pub fn new() -> GetUpdates{
        GetUpdates {
            offset: None,
            limit: None,
            timeout: None,
            allowed_updates: None,
        }
    }
}

#[derive(Serialize, Debug, Clone)]
pub struct SendMessage<'s> {
    pub chat_id: ChatId,

    pub text: Cow<'s, str>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse_mode: Option<ParseMode>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_web_page_preview: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_notification: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_to_message_id: Option<i64>,
}

impl<'s> SendMessage<'s> {
    pub fn new<>(chat_id: ChatId, text: impl Into<Cow<'s, str>>) -> SendMessage<'s> {
        SendMessage {
            chat_id,
            text: text.into(),
            parse_mode: None,
            disable_web_page_preview: None,
            disable_notification: None,
            reply_to_message_id: None,
        }
    }
}

#[derive(Serialize, Debug, Clone, Copy)]
#[serde(rename_all = "snake_case")]
pub enum AllowedUpdate {
    Message,
    EditedMessage,
    ChannelPost,
    EditedChannelPost,
    InlineQuery,
    ChosenInlineResult,
    CallbackQuery,
    ShippingQuery,
    PreCheckoutQuery,
}

#[derive(Serialize, Debug, Clone)]
#[serde(untagged)]
pub enum ChatId {
    Id(i64),
    Username(String)
}

#[derive(Serialize, Debug, Clone, Copy)]
pub enum ParseMode {
    Html,
    Markdown
}