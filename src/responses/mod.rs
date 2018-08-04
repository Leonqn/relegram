use responses::message::Message;

pub mod message;
pub mod chat;
pub mod channel;
pub mod user;
pub mod raw;

#[derive(Clone)]
pub struct Update {
    pub id: i32,
    pub kind: UpdateKind,
}

#[derive(Clone)]
pub enum UpdateKind {
    Message(Message),
    EditedMessage(Message),
}

impl From<raw::Update> for Update {
    fn from(_: raw::Update) -> Self {
        unimplemented!()
    }
}