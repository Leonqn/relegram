pub mod send_message;
pub mod get_updates;
pub mod get_me;
pub mod forward_message;
pub mod chat_id;
pub mod send_media_group;
pub mod input_media;
pub mod get_file;
pub(crate) trait Request {
    fn method(&self) -> &'static str;
}