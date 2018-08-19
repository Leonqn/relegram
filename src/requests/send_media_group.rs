use requests::chat_id::ChatId;
use requests::input_media::InputMediaPhoto;
use requests::input_media::InputMediaVideo;
use std::ops::Not;
use requests::Request;

#[derive(Serialize, Debug, Clone)]
pub struct SendMediaGroupRequest {
    pub chat_id: ChatId,
    pub media: Vec<InputMediaGroup>,
    #[serde(skip_serializing_if = "Not::not")]
    pub disable_notification: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_to_message_id: Option<i64>,
}

#[serde(tag = "type")]
#[derive(Serialize, Debug, Clone)]
pub enum InputMediaGroup {
    Photo(InputMediaPhoto),
    Video(InputMediaVideo),
}

impl Request for SendMediaGroupRequest {
    fn method(&self) -> &'static str {
        "sendMediaGroup"
    }
}