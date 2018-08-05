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
            match (update.message, update.edited_message, update.channel_post, update.edited_channel_post) {
                (Some(msg), None, None, None) =>
                    TryFrom::try_from(msg).map(UpdateKind::Message).map_err(From::from),
                (None, Some(msg), None, None) =>
                    TryFrom::try_from(msg).map(UpdateKind::EditedMessage).map_err(From::from),
                (None, None, Some(msg), None) =>
                    TryFrom::try_from(msg).map(UpdateKind::Message).map_err(From::from),
                (None, None, None, Some(msg)) =>
                    TryFrom::try_from(msg).map(UpdateKind::EditedMessage).map_err(From::from),
                _ =>
                    Err(UnexpectedUpdate::Unsupported)
            };
        message
            .map(|x| Update { id, kind: x })
            .map_err(|x| UnexpectedResponse::UnexpectedUpdate { id, kind: x })
    }
}