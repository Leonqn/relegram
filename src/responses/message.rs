use chrono::prelude::*;
use ::responses::raw;
use super::chat::*;
use super::channel::*;
use error::*;
use std::convert::TryFrom;

pub use self::raw::message::{Audio,
                             Voice,
                             Document,
                             Animation,
                             Game,
                             PhotoSize,
                             Sticker,
                             Video,
                             VideoNote,
                             Contact,
                             Location,
                             Venue,
                             Invoice,
                             SuccessfulPayment,
                             PassportData};
use responses::user::User;

#[derive(Clone, Debug)]
pub struct Message {
    pub id: i64,
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
        original_message_id: i64,
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
    Unknown { typ: String, offset: i64, length: i64 },
}

#[derive(Clone, Debug)]
pub struct Caption {
    pub caption: String,
    pub entities: Option<Vec<MessageEntity>>,
}

#[derive(Clone, Debug)]
pub enum MessageKind {
    Text { text: String, entities: Option<Vec<MessageEntity>> },
    Audio { audio: Audio },
    Document { document: Document, caption: Option<Caption> },
    Animation { animation: Animation, caption: Option<Caption> },
    Game { game: Game },
    Photo { photo: Vec<PhotoSize>, caption: Option<Caption>, media_group_id: Option<String> },
    Sticker { sticker: Sticker },
    Video { video: Video, caption: Option<Caption>, media_group_id: Option<String> },
    Voice { voice: Voice, caption: Option<Caption> },
    VideoNote { video_note: VideoNote },
    Contact { contact: Contact },
    Location { location: Location },
    Venue { venue: Venue },
    NewChatMembers { members: Vec<User> },
    LeftChatMember { member: User },
    NewChatTitle { title: String },
    NewChatPhoto { photo: Vec<PhotoSize> },
    DeleteChatPhoto,
    GroupChatCreated,
    SuperGroupChatCreated,
    ChannelChatCreated,
    MigrateToChatId { id: i64 },
    MigrateFromChatId { id: i64 },
    PinnedMessage { message: Box<Message> },
    Invoice { invoice: Invoice },
    SuccessfulPayment { successful_payment: SuccessfulPayment },
    ConnectedWebsite { connected_website: String },
    PassportData { passport_data: PassportData },
}

impl TryFrom<raw::message::Message> for Message {
    type Error = UnexpectedResponse;

    fn try_from(message: raw::message::Message) -> Result<Self, UnexpectedResponse> {
        {
            fn try_into_forward(from: Option<raw::user::User>,
                                chat: Option<raw::chat::Chat>,
                                id: Option<i64>,
                                sign: Option<String>,
                                date: Option<i64>) -> Result<Option<Forward>, UnexpectedResponse> {
                match (from, chat, id, sign, date) {
                    (None, Some(chat), Some(id), sign, Some(date)) =>
                        TryFrom::try_from(chat)
                            .map(|channel| Some(Forward {
                                original_date: Utc.timestamp(date as i64, 0),
                                from: ForwardFrom::Channel {
                                    channel,
                                    original_message_id: id,
                                    original_signature: sign,
                                },
                            })),
                    (Some(user), None, None, None, Some(date)) =>
                        Ok(Some(Forward {
                            original_date: Utc.timestamp(date as i64, 0),
                            from: ForwardFrom::User(user),
                        })),
                    (None, None, None, None, None) =>
                        Ok(None),

                    _ =>
                        Err(UnexpectedResponse::ConvertError(String::from("Wrong forward arguments. Expected one of this: chat, id, sign, date or user")))
                }
            }

            fn try_into_reply(reply: Option<Box<raw::message::Message>>) -> Result<Option<Message>, UnexpectedResponse> {
                match reply {
                    Some(x) => TryFrom::try_from(*x).map(Some),
                    _ => Ok(None)
                }
            }

            fn try_into_message_from(from: Option<raw::user::User>, chat: raw::chat::Chat, signature: Option<String>) -> Result<MessageFrom, UnexpectedResponse> {
                match from {
                    Some(x) =>
                        TryFrom::try_from(chat).map(|chat| MessageFrom::User { from: x, chat }),
                    None =>
                        TryFrom::try_from(chat).map(|channel| MessageFrom::Channel { channel, signature })
                }
            }

            fn try_into_message_kind(text: Option<String>,
                                     media_group_id: Option<String>,
                                     entities: Option<Vec<raw::message::MessageEntity>>,
                                     caption: Option<String>,
                                     caption_entities: Option<Vec<raw::message::MessageEntity>>,
                                     audio: Option<raw::message::Audio>,
                                     document: Option<raw::message::Document>,
                                     animation: Option<raw::message::Animation>,
                                     game: Option<raw::message::Game>,
                                     photo: Option<Vec<raw::message::PhotoSize>>,
                                     sticker: Option<raw::message::Sticker>,
                                     video: Option<raw::message::Video>,
                                     voice: Option<raw::message::Voice>,
                                     video_note: Option<raw::message::VideoNote>,
                                     contact: Option<raw::message::Contact>,
                                     location: Option<raw::message::Location>,
                                     venue: Option<raw::message::Venue>,
                                     new_chat_members: Option<Vec<raw::user::User>>,
                                     left_chat_member: Option<raw::user::User>,
                                     new_chat_title: Option<String>,
                                     new_chat_photo: Option<Vec<raw::message::PhotoSize>>,
                                     delete_chat_photo: Option<bool>,
                                     group_chat_created: Option<bool>,
                                     supergroup_chat_created: Option<bool>,
                                     channel_chat_created: Option<bool>,
                                     migrate_to_chat_id: Option<i64>,
                                     migrate_from_chat_id: Option<i64>,
                                     pinned_message: Option<Box<raw::message::Message>>,
                                     invoice: Option<raw::message::Invoice>,
                                     successful_payment: Option<raw::message::SuccessfulPayment>,
                                     connected_website: Option<String>,
                                     passport_data: Option<raw::message::PassportData>,
            ) -> Result<MessageKind, UnexpectedResponse> {
                fn into_entities(text: &str, entities: Option<Vec<raw::message::MessageEntity>>) -> Option<Vec<MessageEntity>> {
                    entities.map(|entities| entities.into_iter().map(|entity| {
                        let captured = String::from(&text[(entity.offset as usize)..(entity.offset as usize + entity.length as usize)]);
                        match entity.typ.as_ref() {
                            "mention" =>
                                MessageEntity::Mention(captured),
                            "hashtag" =>
                                MessageEntity::Hashtag(captured),
                            "cashtag" =>
                                MessageEntity::Cashtag(captured),
                            "bot_command" =>
                                MessageEntity::BotCommand(captured),
                            "url" =>
                                MessageEntity::Url(captured),
                            "email" =>
                                MessageEntity::Email(captured),
                            "phone_number" =>
                                MessageEntity::PhoneNumber(captured),
                            "bold" =>
                                MessageEntity::Bold(captured),
                            "italic" =>
                                MessageEntity::Italic(captured),
                            "code" =>
                                MessageEntity::Code(captured),
                            "pre" =>
                                MessageEntity::Pre(captured),
                            "text_link" =>
                                if let Some(link) = entity.url {
                                    MessageEntity::TextLink { text: captured, link }
                                } else {
                                    MessageEntity::Unknown {
                                        typ: entity.typ,
                                        offset: entity.offset,
                                        length: entity.length,
                                    }
                                }
                            "text_mention" =>
                                if let Some(user) = entity.user {
                                    MessageEntity::TextMention { mention: captured, user: From::from(user) }
                                } else {
                                    MessageEntity::Unknown {
                                        typ: entity.typ,
                                        offset: entity.offset,
                                        length: entity.length,
                                    }
                                }
                            _ =>
                                MessageEntity::Unknown {
                                    typ: entity.typ,
                                    offset: entity.offset,
                                    length: entity.length,
                                }
                        }
                    }).collect())
                }
                let caption = caption.map(|caption| {
                    let entities = into_entities(&caption, caption_entities);
                    Caption { caption, entities }
                });

                if let Some(text) = text {
                    let entities = into_entities(&text, entities);
                    return Ok(MessageKind::Text { text, entities });
                }
                if let Some(audio) = audio {
                    return Ok(MessageKind::Audio { audio });
                }
                if let Some(document) = document {
                    return Ok(MessageKind::Document { document, caption });
                }
                if let Some(animation) = animation {
                    return Ok(MessageKind::Animation { animation, caption });
                }
                if let Some(game) = game {
                    return Ok(MessageKind::Game { game });
                }
                if let Some(photo) = photo {
                    return Ok(MessageKind::Photo { photo, media_group_id, caption });
                }
                if let Some(sticker) = sticker {
                    return Ok(MessageKind::Sticker { sticker });
                }
                if let Some(video) = video {
                    return Ok(MessageKind::Video { video, media_group_id, caption });
                }
                if let Some(voice) = voice {
                    return Ok(MessageKind::Voice { voice, caption });
                }
                if let Some(video_note) = video_note {
                    return Ok(MessageKind::VideoNote { video_note });
                }
                if let Some(contact) = contact {
                    return Ok(MessageKind::Contact { contact });
                }
                if let Some(location) = location {
                    return Ok(MessageKind::Location { location });
                }
                if let Some(venue) = venue {
                    return Ok(MessageKind::Venue { venue });
                }
                if let Some(new_chat_members) = new_chat_members {
                    return Ok(MessageKind::NewChatMembers { members: new_chat_members });
                }
                if let Some(left_chat_member) = left_chat_member {
                    return Ok(MessageKind::LeftChatMember { member: left_chat_member });
                }
                if let Some(new_chat_title) = new_chat_title {
                    return Ok(MessageKind::NewChatTitle { title: new_chat_title });
                }
                if let Some(new_chat_photo) = new_chat_photo {
                    return Ok(MessageKind::NewChatPhoto { photo: new_chat_photo });
                }
                if let Some(_) = delete_chat_photo {
                    return Ok(MessageKind::DeleteChatPhoto);
                }
                if let Some(_) = group_chat_created {
                    return Ok(MessageKind::GroupChatCreated);
                }
                if let Some(_) = supergroup_chat_created {
                    return Ok(MessageKind::SuperGroupChatCreated);
                }
                if let Some(_) = channel_chat_created {
                    return Ok(MessageKind::ChannelChatCreated);
                }
                if let Some(migrate_to_chat_id) = migrate_to_chat_id {
                    return Ok(MessageKind::MigrateToChatId { id: migrate_to_chat_id });
                }
                if let Some(migrate_from_chat_id) = migrate_from_chat_id {
                    return Ok(MessageKind::MigrateFromChatId { id: migrate_from_chat_id });
                }
                if let Some(pinned_message) = pinned_message {
                    return TryFrom::try_from(*pinned_message).map(|x| MessageKind::PinnedMessage { message: Box::new(x) });
                }
                if let Some(invoice) = invoice {
                    return Ok(MessageKind::Invoice { invoice });
                }
                if let Some(successful_payment) = successful_payment {
                    return Ok(MessageKind::SuccessfulPayment { successful_payment });
                }
                if let Some(connected_website) = connected_website {
                    return Ok(MessageKind::ConnectedWebsite { connected_website });
                }
                if let Some(passport_data) = passport_data {
                    return Ok(MessageKind::PassportData { passport_data });
                }

                return Err(UnexpectedResponse::ConvertError(String::from("Tried all kind of messages. If this happens, probably there is bug in lib")));
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
                    caption,
                    caption_entities,
                    text,
                    audio,
                    document, animation, game,
                    photo, sticker, video, voice,
                    video_note, contact, location, venue,
                    new_chat_members, left_chat_member, new_chat_title,
                    new_chat_photo, delete_chat_photo, group_chat_created, supergroup_chat_created,
                    channel_chat_created, migrate_to_chat_id, migrate_from_chat_id, pinned_message, invoice,
                    successful_payment, connected_website, passport_data,
                } =>
                    try_into_forward(forward_from, forward_from_chat, forward_from_message_id, forward_signature, forward_date)
                        .and_then(move |forward| try_into_reply(reply_to_message).map(|reply| (forward, reply)))
                        .and_then(|(forward, reply)|
                            try_into_message_kind(text, media_group_id, entities, caption,
                                                  caption_entities, audio,
                                                  document, animation, game,
                                                  photo, sticker, video, voice,
                                                  video_note, contact, location, venue,
                                                  new_chat_members, left_chat_member, new_chat_title,
                                                  new_chat_photo, delete_chat_photo, group_chat_created, supergroup_chat_created,
                                                  channel_chat_created, migrate_to_chat_id, migrate_from_chat_id, pinned_message, invoice,
                                                  successful_payment, connected_website, passport_data)
                                .map(|kind| (forward, reply, kind)))
                        .and_then(|(forward, reply, kind)| try_into_message_from(from, chat, author_signature).map(|from| (forward, reply, kind, from)))
                        .map(|(forward, reply, kind, from)|
                            Message {
                                id: message_id,
                                date: Utc.timestamp(date as i64, 0),
                                from,
                                forward,
                                edit_date: edit_date.map(|x| Utc.timestamp(x as i64, 0)),
                                reply_to_message: reply.map(Box::new),
                                kind,
                            }),
            }
        }
    }
}