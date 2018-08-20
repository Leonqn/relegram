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
pub struct Text {
    pub text: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse_mode: Option<ParseMode>,

    #[serde(skip_serializing_if = "Not::not")]
    pub disable_web_page_preview: bool,
}


#[derive(Serialize, Debug, Clone)]
pub struct Photo {
    pub photo: File,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse_mode: Option<ParseMode>,
}

#[derive(Serialize, Debug, Clone)]
pub struct Audio {
    pub audio: File,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse_mode: Option<ParseMode>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub performer: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
}

#[derive(Serialize, Debug, Clone)]
pub struct Document {
    pub document: File,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse_mode: Option<ParseMode>,
}

#[derive(Serialize, Debug, Clone)]
pub struct Video {
    pub video: File,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub with: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub height: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse_mode: Option<ParseMode>,
    #[serde(skip_serializing_if = "Not::not")]
    pub supports_streaming: bool,
}

#[derive(Serialize, Debug, Clone)]
pub struct Animation {
    pub animation: File,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub with: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub height: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse_mode: Option<ParseMode>,
}

#[derive(Serialize, Debug, Clone)]
pub struct Voice {
    pub voice: File,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse_mode: Option<ParseMode>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<i64>,
}

#[derive(Serialize, Debug, Clone)]
pub struct VideoNote {
    pub video_note: File,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub length: Option<i64>,
}

#[derive(Serialize, Debug, Clone)]
pub struct Venue {
    pub latitude: f32,
    pub longitude: f32,
    pub title: String,
    pub address: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub foursquare_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub foursquare_type: Option<String>,
}

#[derive(Serialize, Debug, Clone)]
pub struct Location {
    pub latitude: f32,
    pub longitude: f32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub live_period: Option<i64>,
}

#[derive(Serialize, Debug, Clone)]
pub struct Contact {
    pub phone_number: String,
    pub first_name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_name: Option<String>,
    pub vcard: Option<String>,
}

#[derive(Serialize, Debug, Clone)]
#[serde(untagged)]
pub enum SendMessageKind {
    Text(Text),
    Photo(Photo),
    Audio(Audio),
    Document(Document),
    Video(Video),
    Animation(Animation),
    Voice(Voice),
    VideoNote(VideoNote),
    Location(Location),
    Venue(Venue),
    Contact(Contact),
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
pub struct InlineKeyboard {
    pub inline_keyboard: Vec<Vec<InlineKeyboardButton>>
}

#[derive(Serialize, Debug, Clone)]
pub struct ReplyKeyboardMarkup {
    pub keyboard: Vec<Vec<KeyboardButton>>,
    #[serde(skip_serializing_if = "Not::not")]
    pub resize_keyboard: bool,
    #[serde(skip_serializing_if = "Not::not")]
    pub one_time_keyboard: bool,
    #[serde(skip_serializing_if = "Not::not")]
    pub selective: bool,
}

#[derive(Serialize, Debug, Clone)]
pub struct ReplyKeyboardRemove {
    #[serde(skip_serializing_if = "Not::not")]
    pub remove_keyboard: bool,
    #[serde(skip_serializing_if = "Not::not")]
    pub selective: bool,
}

#[derive(Serialize, Debug, Clone)]
pub struct ForceReply {
    #[serde(skip_serializing_if = "Not::not")]
    pub force_reply: bool,
    #[serde(skip_serializing_if = "Not::not")]
    pub selective: bool,
}

#[derive(Serialize, Debug, Clone)]
#[serde(untagged)]
pub enum ReplyMarkup {
    InlineKeyboard(InlineKeyboard),
    ReplyKeyboardMarkup(ReplyKeyboardMarkup),
    ReplyKeyboardRemove(ReplyKeyboardRemove),
    ForceReply(ForceReply),
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
    pub text: String,
    #[serde(skip_serializing_if = "Not::not")]
    pub request_contact: bool,
    #[serde(skip_serializing_if = "Not::not")]
    pub request_location: bool,
}


impl SendMessageRequest {
    pub fn new(chat_id: ChatId, kind: SendMessageKind) -> SendMessageRequest {
        SendMessageRequest {
            chat_id,
            kind,
            disable_notification: false,
            reply_to_message_id: None,
            reply_markup: None,
        }
    }
}

impl Text {
    pub fn new(text: String) -> Text {
        Text {
            text,
            parse_mode: None,
            disable_web_page_preview: false,
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
            SendMessageKind::Venue { .. } => "sendVenue",
            SendMessageKind::Contact { .. } => "sendContact",
        }
    }
}