use responses::raw::chat::Chat;
use error::UnexpectedResponse;
use std::convert::TryFrom;

#[derive(Clone, Debug)]
pub struct Channel {
    pub id: i64,
    pub title: String,
    pub username: Option<String>,
}

impl TryFrom<Chat> for Channel {
    type Error = UnexpectedResponse;

    fn try_from(chat: Chat) -> Result<Self, UnexpectedResponse> {
        match (chat.id, chat.title, chat.username, chat.typ.as_ref()) {
            (id, Some(title), username, "channel") =>
                Ok(Channel {
                    id,
                    title,
                    username,
                }),
            _ =>
                Err(UnexpectedResponse::ConvertError(String::from("Wrong chat. Excepted channel")))
        }
    }
}