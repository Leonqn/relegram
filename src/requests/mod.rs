pub mod send_message;
pub mod get_updates;
pub mod get_me;

pub trait Request {
    fn method(&self) -> &'static str;
}