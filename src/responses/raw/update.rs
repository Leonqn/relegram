use responses::raw::message::Message;

#[derive(Deserialize, Debug)]
pub struct Update {
    pub update_id: i32,
    pub message: Option<Message>,
    pub edited_message: Option<Message>,
    pub channel_post: Option<Message>,
    pub edited_channel_post: Option<Message>
}