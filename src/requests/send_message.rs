use std::ops::Not;
use requests::Request;
use requests::chat_id::ChatId;

#[derive(Serialize, Debug, Clone)]
pub struct SendMessageRequest {
    pub chat_id: ChatId,

    #[serde(flatten)]
    pub kind: SendMessageKind,

    #[serde(skip_serializing_if = "Not::not")]
    pub disable_notification: bool,

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
        #[serde(skip_serializing_if = "Not::not")]
        disable_web_page_preview: bool,
    },
    Photo {
        photo: File,
        #[serde(skip_serializing_if = "Option::is_none")]
        caption: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        parse_mode: Option<ParseMode>,
    },
    Audio {
        audio: File,
        #[serde(skip_serializing_if = "Option::is_none")]
        caption: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        parse_mode: Option<ParseMode>,
        #[serde(skip_serializing_if = "Option::is_none")]
        duration: Option<i32>,
        #[serde(skip_serializing_if = "Option::is_none")]
        performer: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        title: Option<String>,
    },
    Document {
        document: File,
        #[serde(skip_serializing_if = "Option::is_none")]
        caption: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        parse_mode: Option<ParseMode>,
    },
    Video {
        video: File,
        #[serde(skip_serializing_if = "Option::is_none")]
        duration: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none")]
        with: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none")]
        height: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none")]
        caption: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        parse_mode: Option<ParseMode>,
        #[serde(skip_serializing_if = "Not::not")]
        supports_streaming: bool,
    },
    Animation {
        animation: File,
        #[serde(skip_serializing_if = "Option::is_none")]
        duration: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none")]
        with: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none")]
        height: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none")]
        caption: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        parse_mode: Option<ParseMode>,
    },
    Voice {
        voice: File,
        #[serde(skip_serializing_if = "Option::is_none")]
        caption: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        parse_mode: Option<ParseMode>,
        #[serde(skip_serializing_if = "Option::is_none")]
        duration: Option<i64>,
    },
    VideoNote {
        video_note: File,
        #[serde(skip_serializing_if = "Option::is_none")]
        duration: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none")]
        length: Option<i64>,
    },
    Location {
        latitude: f32,
        longitude: f32,
        #[serde(skip_serializing_if = "Option::is_none")]
        live_period: Option<i64>,
    },
    Venue {
        latitude: f32,
        longitude: f32,
        title: String,
        address: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        foursquare_id: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        foursquare_type: Option<String>,
    },
    Contact {
        phone_number: String,
        first_name: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        last_name: Option<String>,
        vcard: Option<String>,
    },
}

#[derive(Serialize, Debug, Clone)]
#[serde(untagged)]
pub enum File {
    FileId(String),
    Url(String),
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
        #[serde(skip_serializing_if = "Not::not")]
        selective: bool,
    },
    ReplyKeyboardRemove {
        #[serde(skip_serializing_if = "Not::not")]
        remove_keyboard: bool,
        #[serde(skip_serializing_if = "Not::not")]
        selective: bool,
    },
    ForceReply {
        #[serde(skip_serializing_if = "Not::not")]
        force_reply: bool,
        #[serde(skip_serializing_if = "Not::not")]
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
            disable_notification: false,
            reply_to_message_id: None,
            reply_markup: None,
        }
    }
}

impl Request for SendMessageRequest {
    fn method(&self) -> &'static str {
        match self.kind {
            SendMessageKind::Text { .. } => "sendMessage",
            SendMessageKind::Photo { .. } => "sendPhoto",
            SendMessageKind::Audio { .. } => "sendAudio",
            SendMessageKind::Document { .. } => "sendDocument",
            SendMessageKind::Video { .. } => "sendVideo",
            SendMessageKind::Animation { .. } => "sendAnimation",
            SendMessageKind::Voice { .. } => "sendVoice",
            SendMessageKind::VideoNote { .. } => "sendVideoNote",
            SendMessageKind::Location { .. } => "sendLocation",
            SendMessageKind::Venue  { .. } => "sendVenue",
            SendMessageKind::Contact { .. } => "sendContact",
        }
    }
}