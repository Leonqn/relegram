use responses::raw::message::Message;
use responses::raw::queries::CallbackQuery;

#[derive(Deserialize, Debug)]
pub struct Update {
    pub update_id: i64,
    pub message: Option<Message>,
    pub edited_message: Option<Message>,
    pub channel_post: Option<Message>,
    pub edited_channel_post: Option<Message>,
    pub callback_query: Option<CallbackQuery>
}