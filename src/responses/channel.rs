use responses::raw::chat;

#[derive(Clone, Debug)]
pub struct Channel {
    pub id: i64,
}

impl From<chat::Chat> for Channel {
    fn from(chat: chat::Chat) -> Self {
        unimplemented!()
    }
}