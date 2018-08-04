use chrono::prelude::*;
use ::responses::raw;
use super::chat::*;
use super::channel::*;
use super::user::*;
use error::*;
use std::convert::TryFrom;

#[derive(Clone)]
pub struct Message {
    pub id: i32,
    pub date: DateTime<Utc>,
    pub from: MessageFrom,
    pub forward: Option<Forward>,
    pub edit_date: Option<DateTime<Utc>>,
    pub reply_to_message: Option<Box<Message>>,
    pub kind: MessageKind,
}

#[derive(Clone)]
pub enum MessageFrom {
    Channel {
        channel: Channel,
        signature: Option<String>,
    },
    User {
        from: User,
        chat: Chat,
    },
}


#[derive(Clone)]
pub struct Forward {
    pub original_date: DateTime<Utc>,
    pub from: ForwardFrom,
}

#[derive(Clone)]
pub enum ForwardFrom {
    User(User),
    Channel {
        channel: Channel,
        original_message_id: i32,
        original_signature: Option<String>,
    },
}

#[derive(Clone)]
pub enum MessageKind {
    Text(String),
    Unknown
}

#[derive(Clone)]
pub enum ServiceMessage {}


impl TryFrom<raw::message::Message> for Message {
    type Error = Error;

    fn try_from(message: raw::message::Message) -> Result<Self, Error> {
        {
            fn try_into_forward(from: Option<raw::user::User>, chat: Option<raw::chat::Chat>, id: Option<i32>, sign: Option<String>, date: Option<i32>) -> Result<Option<Forward>, Error> {
                match (from, chat, id, sign, date) {
                    (None, Some(chat), Some(id), sign, Some(date)) =>
                        Ok(Some(Forward {
                            original_date: Utc.timestamp(date as i64, 0),
                            from: ForwardFrom::Channel {
                                channel: From::from(chat),
                                original_message_id: id,
                                original_signature: sign,
                            },
                        })),

                    (Some(user), None, None, None, Some(date)) =>
                        Ok(Some(Forward {
                            original_date: Utc.timestamp(date as i64, 0),
                            from: ForwardFrom::User(From::from(user)),
                        })),

                    (None, None, None, None, None) =>
                        Ok(None),

                    _ =>
                        Err(Error::BadMessage(BadMessage::WrongForwardArguments))
                }
            }

            fn try_into_reply(reply: Option<Box<raw::message::Message>>) -> Result<Option<Message>, Error> {
                match reply {
                    Some(x) => TryFrom::try_from(*x).map(Some),
                    _ => Ok(None)
                }
            }

            fn into_message_from(from: Option<raw::user::User>, chat: raw::chat::Chat, signature: Option<String>) -> MessageFrom {
                match from {
                    Some(x) =>
                        MessageFrom::User { from: From::from(x), chat: From::from(chat) },
                    None =>
                        MessageFrom::Channel { channel: From::from(chat), signature }
                }
            }

            fn into_message_kind(text: Option<String>, voice: Option<raw::message::Voice>) -> MessageKind {
                if let Some(text) = text {
                    return MessageKind::Text(text);
                }
                MessageKind::Unknown
            }

            match message {
                raw::message::Message {
                    message_id,
                    from,
                    date,
                    chat,
                    forward_from,
                    forward_from_chat,
                    forward_from_message_id,
                    forward_signature,
                    forward_date,
                    reply_to_message,
                    edit_date,
                    media_group_id,
                    author_signature,
                    text,
                    voice,
                } =>
                    try_into_forward(forward_from, forward_from_chat, forward_from_message_id, forward_signature, forward_date)
                        .and_then(move |forward| try_into_reply(reply_to_message).map(|reply| (forward, reply)))
                        .map(|(forward, reply)|
                            Message {
                                id: message_id,
                                date: Utc.timestamp(date as i64, 0),
                                from: into_message_from(from, chat, author_signature),
                                forward,
                                edit_date: edit_date.map(|x| Utc.timestamp(x as i64, 0)),
                                reply_to_message: reply.map(Box::new),
                                kind: into_message_kind(text, voice),
                            }),
                _ => unimplemented!()
            }
        }
    }
}