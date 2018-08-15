use responses::message::Message;
use std::convert::TryFrom;
use error::*;
use super::raw;

#[derive(Clone, Debug)]
pub struct Update {
    pub id: i32,
    pub kind: UpdateKind,
}

#[derive(Clone, Debug)]
pub enum UpdateKind {
    Message(Message),
    EditedMessage(Message),
}

impl TryFrom<raw::update::Update> for Update {
    type Error = UnexpectedResponse;

    fn try_from(update: raw::update::Update) -> Result<Self, UnexpectedResponse> {
        let id = update.update_id;
        let message =
            match update {
                raw::update::Update { message: Some(msg), .. } =>
                    TryFrom::try_from(msg).map(UpdateKind::Message),

                raw::update::Update { edited_message: Some(msg), .. } =>
                    TryFrom::try_from(msg).map(UpdateKind::EditedMessage),

                raw::update::Update { channel_post: Some(post), .. } =>
                    TryFrom::try_from(post).map(UpdateKind::Message),

                raw::update::Update { edited_channel_post: Some(post), .. } =>
                    TryFrom::try_from(post).map(UpdateKind::Message),

                _ =>
                    Err(UnexpectedResponse::Unsupported)
            };
        message
            .map(|x| Update { id, kind: x })
    }
}
