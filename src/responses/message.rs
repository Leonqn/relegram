use chrono::prelude::*;
use ::responses::raw;
use super::chat::*;
use super::channel::*;
use super::user::*;
use error::*;
use std::convert::TryFrom;

#[derive(Clone, Debug)]
pub struct Message {
    pub id: i32,
    pub date: DateTime<Utc>,
    pub from: MessageFrom,
    pub forward: Option<Forward>,
    pub edit_date: Option<DateTime<Utc>>,
    pub reply_to_message: Option<Box<Message>>,
    pub kind: MessageKind,
}

#[derive(Clone, Debug)]
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


#[derive(Clone, Debug)]
pub struct Forward {
    pub original_date: DateTime<Utc>,
    pub from: ForwardFrom,
}

#[derive(Clone, Debug)]
pub enum ForwardFrom {
    User(User),
    Channel {
        channel: Channel,
        original_message_id: i32,
        original_signature: Option<String>,
    },
}

#[derive(Clone, Debug)]
pub enum MessageEntity {
    Mention(String),
    Hashtag(String),
    Cashtag(String),
    BotCommand(String),
    Url(String),
    Email(String),
    PhoneNumber(String),
    Bold(String),
    Italic(String),
    Code(String),
    Pre(String),
    TextLink { text: String, link: String },
    TextMention { mention: String, user: User },
}

#[derive(Clone, Debug)]
pub enum MessageKind {
    Text { text: String, entities: Option<Vec<MessageEntity>> },
}

#[derive(Clone, Debug)]
pub enum ServiceMessage {}


impl TryFrom<raw::message::Message> for Message {
    type Error = UnexpectedMessage;

    fn try_from(message: raw::message::Message) -> Result<Self, UnexpectedMessage> {
        {
            fn try_into_forward(from: Option<raw::user::User>,
                                chat: Option<raw::chat::Chat>,
                                id: Option<i32>,
                                sign: Option<String>,
                                date: Option<i32>) -> Result<Option<Forward>, UnexpectedMessage> {
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
                        Err(UnexpectedMessage::WrongForwardArguments)
                }
            }

            fn try_into_reply(reply: Option<Box<raw::message::Message>>) -> Result<Option<Message>, UnexpectedMessage> {
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

            fn try_into_message_kind(text: Option<String>,
                                     entities: Option<Vec<raw::message::MessageEntity>>,
                                     voice: Option<raw::message::Voice>) -> Result<MessageKind, UnexpectedMessage> {
                if let Some(text) = text {
                    let entities =
                        entities.map(|entities| entities.into_iter().map(|entity| {
                            let captured = String::from(&text[(entity.offset as usize)..(entity.offset as usize + entity.length as usize)]);
                            match entity.typ.as_ref() {
                                "mention" =>
                                    Ok(MessageEntity::Mention(captured)),
                                "hashtag" =>
                                    Ok(MessageEntity::Hashtag(captured)),
                                "cashtag" =>
                                    Ok(MessageEntity::Cashtag(captured)),
                                "bot_command" =>
                                    Ok(MessageEntity::BotCommand(captured)),
                                "url" =>
                                    Ok(MessageEntity::Url(captured)),
                                "email" =>
                                    Ok(MessageEntity::Email(captured)),
                                "phone_number" =>
                                    Ok(MessageEntity::PhoneNumber(captured)),
                                "bold" =>
                                    Ok(MessageEntity::Bold(captured)),
                                "italic" =>
                                    Ok(MessageEntity::Italic(captured)),
                                "code" =>
                                    Ok(MessageEntity::Code(captured)),
                                "pre" =>
                                    Ok(MessageEntity::Pre(captured)),
                                "text_link" =>
                                    if let Some(link) = entity.url {
                                        Ok(MessageEntity::TextLink { text: captured, link })
                                    } else {
                                        Err(UnexpectedMessage::WrongMessageEntity)
                                    }
                                "text_mention" =>
                                    if let Some(user) = entity.user {
                                        Ok(MessageEntity::TextMention { mention: captured, user: From::from(user) })
                                    } else {
                                        Err(UnexpectedMessage::WrongMessageEntity)
                                    }
                                _ =>
                                    Err(UnexpectedMessage::UnsupportedMessageEntity)

                            }
                        }).collect())
                            .map_or(Ok(None), |x: Result<Vec<MessageEntity>, UnexpectedMessage>| x.map(Some));
                    return entities.map(|entities| MessageKind::Text { text, entities })
                }

                Err(UnexpectedMessage::UnsupportedMessageKind)
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
                    entities,
                    text,
                    voice,
                    ..
                } =>
                    try_into_forward(forward_from, forward_from_chat, forward_from_message_id, forward_signature, forward_date)
                        .and_then(move |forward| try_into_reply(reply_to_message).map(|reply| (forward, reply)))
                        .and_then(|(forward, reply)| try_into_message_kind(text, entities, voice).map(|kind| (forward, reply, kind)))
                        .map(|(forward, reply, kind)|
                                 Message {
                                     id: message_id,
                                     date: Utc.timestamp(date as i64, 0),
                                     from: into_message_from(from, chat, author_signature),
                                     forward,
                                     edit_date: edit_date.map(|x| Utc.timestamp(x as i64, 0)),
                                     reply_to_message: reply.map(Box::new),
                                     kind,
            }),
            _ => unimplemented!()
        }
    }
}
}