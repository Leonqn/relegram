use std::ops::Not;
use requests::Request;
use requests::chat_id::ChatId;
use requests::reply_markup::ReplyMarkup;

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
    pub photo: FileKind,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse_mode: Option<ParseMode>,
}

#[derive(Serialize, Debug, Clone)]
pub struct Audio {
    pub audio: FileKind,
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
    pub document: FileKind,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse_mode: Option<ParseMode>,
}

#[derive(Serialize, Debug, Clone)]
pub struct Video {
    pub video: FileKind,
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
    pub animation: FileKind,
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
    pub voice: FileKind,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse_mode: Option<ParseMode>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<i64>,
}

#[derive(Serialize, Debug, Clone)]
pub struct VideoNote {
    pub video_note: FileKind,
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
pub enum FileKind {
    FileId(String),
    Url(String),
}

#[derive(Serialize, Debug, Clone, Copy)]
pub enum ParseMode {
    Html,
    Markdown,
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
            SendMessageKind::Text(_) => "sendMessage",
            SendMessageKind::Photo(_) => "sendPhoto",
            SendMessageKind::Audio(_) => "sendAudio",
            SendMessageKind::Document(_) => "sendDocument",
            SendMessageKind::Video(_) => "sendVideo",
            SendMessageKind::Animation(_) => "sendAnimation",
            SendMessageKind::Voice(_) => "sendVoice",
            SendMessageKind::VideoNote(_) => "sendVideoNote",
            SendMessageKind::Location(_) => "sendLocation",
            SendMessageKind::Venue(_) => "sendVenue",
            SendMessageKind::Contact(_) => "sendContact",
        }
    }
}