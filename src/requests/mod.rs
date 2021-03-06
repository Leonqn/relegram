pub use self::send_message::*;
pub use self::get_updates::*;
pub use self::get_me::*;
pub use self::forward_message::*;
pub use self::chat_id::*;
pub use self::send_media_group::*;
pub use self::input_media::*;
pub use self::get_file::*;
pub use self::reply_markup::*;
pub use self::answer_callback_query::*;
pub use self::chat_action::*;

mod send_message;
mod get_updates;
mod get_me;
mod forward_message;
mod chat_id;
mod send_media_group;
mod input_media;
mod get_file;
mod reply_markup;
mod answer_callback_query;
mod chat_action;
pub(crate) trait Request {
    fn method(&self) -> &'static str;
}