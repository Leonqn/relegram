use std::ops::Not;
use requests::Request;

#[derive(Serialize, Debug, Clone)]
pub struct SendMessageRequest {
    pub chat_id: ChatId,

    #[serde(flatten)]
    pub kind: SendMessageKind,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_notification: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_to_message_id: Option<i64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<ReplyMarkup>,
}

#[derive(Serialize, Debug, Clone)]
#[serde(untagged)]
pub enum SendMessageKind {
    Text {
        text: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        parse_mode: Option<ParseMode>,
        #[serde(skip_serializing_if = "Option::is_none")]
        disable_web_page_preview: Option<bool>,
    },
    Audio {
        audio: File,
        caption: Option<String>,
        parse_mode: Option<ParseMode>,
        duration: Option<i32>,
        performer: Option<String>,
        title: Option<String>,
    },
}

#[derive(Serialize, Debug, Clone)]
#[serde(untagged)]
pub enum File {
    FileId(String),
    Url(String),
}


#[derive(Serialize, Debug, Clone)]
#[serde(untagged)]
pub enum ChatId {
    Id(i64),
    Username(String),
}

#[derive(Serialize, Debug, Clone, Copy)]
pub enum ParseMode {
    Html,
    Markdown,
}

#[derive(Serialize, Debug, Clone)]
#[serde(untagged)]
pub enum ReplyMarkup {
    InlineKeyboard {
        inline_keyboard: Vec<Vec<InlineKeyboardButton>>
    },
    ReplyKeyboardMarkup {
        keyboard: Vec<Vec<KeyboardButton>>,
        #[serde(skip_serializing_if = "Not::not")]
        resize_keyboard: bool,
        #[serde(skip_serializing_if = "Not::not")]
        one_time_keyboard: bool,
        selective: bool,
    },
    ReplyKeyboardRemove {
        #[serde(skip_serializing_if = "Not::not")]
        remove_keyboard: bool,
        selective: bool,
    },
    ForceReply {
        #[serde(skip_serializing_if = "Not::not")]
        force_reply: bool,
        selective: bool,
    },
}

#[derive(Serialize, Debug, Clone)]
pub struct InlineKeyboardButton {
    pub text: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub callback_data: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub switch_inline_query: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub switch_inline_query_current_chat: Option<String>,
    //    pub callback_game: Option<CallbackGame>
    #[serde(skip_serializing_if = "Not::not")]
    pub pay: bool,
}

#[derive(Serialize, Debug, Clone)]
pub struct KeyboardButton {
    text: String,
    #[serde(skip_serializing_if = "Not::not")]
    request_contact: bool,
    #[serde(skip_serializing_if = "Not::not")]
    request_location: bool,
}


impl SendMessageRequest {
    pub fn new<>(chat_id: ChatId, kind: SendMessageKind) -> SendMessageRequest {
        SendMessageRequest {
            chat_id,
            kind,
            disable_notification: None,
            reply_to_message_id: None,
            reply_markup: None,
        }
    }
}

impl Request for SendMessageRequest {
    fn method(&self) -> &'static str {
        match self.kind {
            SendMessageKind::Text { .. } => "sendMessage",
            SendMessageKind::Audio { .. } => "sendAudio"
        }
    }
}