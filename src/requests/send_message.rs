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
pub struct SendText {
    pub text: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse_mode: Option<ParseMode>,
    #[serde(skip_serializing_if = "Not::not")]
    pub disable_web_page_preview: bool,
}

#[derive(Serialize, Debug, Clone)]
pub struct SendPhoto {
    pub photo: FileKind,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse_mode: Option<ParseMode>,
}

#[derive(Serialize, Debug, Clone)]
pub struct SendAudio {
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
pub struct SendDocument {
    pub document: FileKind,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse_mode: Option<ParseMode>,
}

#[derive(Serialize, Debug, Clone)]
pub struct SendVideo {
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
pub struct SendAnimation {
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
pub struct SendVoice {
    pub voice: FileKind,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse_mode: Option<ParseMode>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<i64>,
}

#[derive(Serialize, Debug, Clone)]
pub struct SendVideoNote {
    pub video_note: FileKind,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub length: Option<i64>,
}

#[derive(Serialize, Debug, Clone)]
pub struct SendVenue {
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
pub struct SendLocation {
    pub latitude: f32,
    pub longitude: f32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub live_period: Option<i64>,
}

#[derive(Serialize, Debug, Clone)]
pub struct SendContact {
    pub phone_number: String,
    pub first_name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_name: Option<String>,
    pub vcard: Option<String>,
}

#[derive(Serialize, Debug, Clone)]
#[serde(untagged)]
pub enum SendMessageKind {
    Text(SendText),
    Photo(SendPhoto),
    Audio(SendAudio),
    Document(SendDocument),
    Video(SendVideo),
    Animation(SendAnimation),
    Voice(SendVoice),
    VideoNote(SendVideoNote),
    Location(SendLocation),
    Venue(SendVenue),
    Contact(SendContact),
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

impl SendText {
    pub fn new(text: String) -> SendText {
        SendText {
            text,
            parse_mode: None,
            disable_web_page_preview: false,
        }
    }
}

impl SendPhoto {
    pub fn new(photo: FileKind) -> SendPhoto {
        SendPhoto {
            photo,
            caption: None,
            parse_mode: None,
        }
    }
}

impl SendAnimation {
    pub fn new(animation: FileKind) -> SendAnimation {
        SendAnimation {
            animation,
            duration: None,
            with: None,
            height: None,
            caption: None,
            parse_mode: None,
        }
    }
}

impl SendAudio {
    pub fn new(audio: FileKind) -> SendAudio {
        SendAudio {
            audio,
            caption: None,
            parse_mode: None,
            duration: None,
            performer: None,
            title: None,
        }
    }
}

impl SendContact {
    pub fn new(phone_number: String, first_name: String) -> SendContact {
        SendContact {
            phone_number,
            first_name,
            last_name: None,
            vcard: None,
        }
    }
}

impl SendDocument {
    pub fn new(document: FileKind) -> SendDocument {
        SendDocument {
            document,
            caption: None,
            parse_mode: None,
        }
    }
}

impl SendLocation {
    pub fn new(latitude: f32, longitude: f32) -> SendLocation {
        SendLocation {
            latitude,
            longitude,
            live_period: None,
        }
    }
}

impl SendVideoNote {
    pub fn new(video_note: FileKind) -> SendVideoNote {
        SendVideoNote {
            video_note,
            duration: None,
            length: None,
        }
    }
}

impl SendVenue {
    pub fn new(latitude: f32, longitude: f32, title: String, address: String) -> SendVenue {
        SendVenue {
            latitude,
            longitude,
            title,
            address,
            foursquare_id: None,
            foursquare_type: None,
        }
    }
}

impl SendVideo {
    pub fn new(video: FileKind) -> SendVideo {
        SendVideo {
            video,
            duration: None,
            with: None,
            height: None,
            caption: None,
            parse_mode: None,
            supports_streaming: false,
        }
    }
}

impl SendVoice {
    pub fn new(voice: FileKind) -> SendVoice {
        SendVoice {
            voice,
            caption: None,
            parse_mode: None,
            duration: None,
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