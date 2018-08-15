use super::raw::chat;
use std::convert::TryFrom;
use error::UnexpectedResponse;

#[derive(Clone, Debug)]
pub struct Chat {
    pub id: i64,
    pub kind: ChatKind,
}

#[derive(Clone, Debug)]
pub enum ChatKind {
    Private {
        username: Option<String>,
        first_name: String,
        last_name: Option<String>,
    },
    Group {
        title: String,
        all_members_are_administrators: bool,
    },
    SuperGroup {
        title: String,
        username: Option<String>,
    },
}

impl TryFrom<chat::Chat> for Chat {
    type Error = UnexpectedResponse;

    fn try_from(chat: chat::Chat) -> Result<Self, UnexpectedResponse> {
        let chat =
            match (chat.id, chat.username, chat.first_name, chat.last_name, chat.all_members_are_administrators, chat.title, chat.typ.as_ref()) {
                (id, username, Some(first_name), last_name, _, _, "private") =>
                    Chat {
                        id,
                        kind: ChatKind::Private {
                            username,
                            first_name,
                            last_name,
                        },
                    },
                (id, _, _, _, all_members_are_administrators, Some(title), "group") =>
                    Chat {
                        id,
                        kind: ChatKind::Group {
                            title,
                            all_members_are_administrators: all_members_are_administrators.unwrap_or(false),
                        },
                    },
                (id, username, _, _, _, Some(title), "supergroup") =>
                    Chat {
                        id,
                        kind: ChatKind::SuperGroup {
                            title,
                            username,
                        },
                    },
                _ =>
                    return Err(UnexpectedResponse::ConvertError(String::from("Wrong chat. Excepted one of this: private, group or supergroup")))
            };
        Ok(chat)
    }
}