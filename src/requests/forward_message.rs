use requests::chat_id::ChatId;
use std::ops::Not;
use requests::Request;

#[derive(Serialize, Debug, Clone)]
pub struct ForwardMessageRequest {
    pub chat_id: ChatId,

    pub from_chat_id: ChatId,

    #[serde(skip_serializing_if = "Not::not")]
    pub disable_notification: bool,

    pub message_id: i64
}


impl Request for ForwardMessageRequest {
    fn method(&self) -> &'static str {
        "forwardMessage"
    }
}