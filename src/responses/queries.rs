use responses::user::User;
use responses::message::Message;
use super::raw::queries;
use error::UnexpectedResponse;
use TryFrom;

#[derive(Debug, Clone)]
pub struct CallbackQuery {
    pub id: String,
    pub from: User,
    pub message: Option<Message>,
    pub inline_message_id: Option<String>,
    pub chat_instance: String,
    pub data: Option<String>,
    pub game_short_name: Option<String>,
}

impl TryFrom<queries::CallbackQuery> for CallbackQuery {
    type Error = UnexpectedResponse;

    fn try_from(value: queries::CallbackQuery) -> Result<Self, UnexpectedResponse> {
        match value {
            queries::CallbackQuery { id, from, message, inline_message_id, data, game_short_name, chat_instance } => {
                let message =
                    message
                        .map(TryFrom::try_from)
                        .map_or(Ok(None), |x| x.map(Some));
                message.map(|message| CallbackQuery {
                    id,
                    from,
                    message,
                    inline_message_id,
                    chat_instance,
                    data,
                    game_short_name,
                })
            }
        }
    }
}