use responses::raw::message::Message;

#[derive(Deserialize, Debug)]
pub struct Chat {
    pub id: i64,
    #[serde(rename = "type")]
    pub typ: String,
    pub title: Option<String>,
    pub username: Option<String>,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub all_members_are_administrators: Option<bool>,
    pub photo: Option<ChatPhoto>,
    pub description: Option<String>,
    pub invite_link: Option<String>,
    pub pinned_message: Option<Box<Message>>,
    pub sticker_set_name: Option<String>,
    pub can_set_sticker_set: Option<bool>
}

#[derive(Deserialize, Debug)]
pub struct ChatPhoto {
    pub small_file_id: String,
    pub big_file_id: String
}