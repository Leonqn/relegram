use requests::chat_id::ChatId;
use requests::Request;

#[derive(Serialize, Debug, Clone)]
pub struct SendChatAction {
    pub chat_id: ChatId,
    pub action: ChatAction,
}

impl Request for SendChatAction {
    fn method(&self) -> &'static str {
        "sendChatAction"
    }
}

#[derive(Serialize, Debug, Clone)]
pub enum ChatAction {
    Typing
}