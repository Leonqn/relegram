use super::raw::chat;

#[derive(Clone, Debug)]
pub struct Chat {
    pub id: i64
}


impl From<chat::Chat> for Chat {
    fn from(chat: chat::Chat) -> Self {
        unimplemented!()
    }
}

