use responses::message::Message;
use error::*;
use super::raw;

pub use super::raw::queries::{InlineQuery, ChosenInlineResult, ShippingQuery, PreCheckoutQuery};
use responses::queries::CallbackQuery;
use try_from::TryFrom;

#[derive(Clone, Debug)]
pub struct Update {
    pub id: i64,
    pub kind: UpdateKind,
}

#[derive(Clone, Debug)]
pub enum UpdateKind {
    Message(Message),
    EditedMessage(Message),
    InlineQuery(InlineQuery),
    ChosenInlineResult(ChosenInlineResult),
    CallbackQuery(CallbackQuery),
    ShippingQuery(ShippingQuery),
    PreCheckoutQuery(PreCheckoutQuery)
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
