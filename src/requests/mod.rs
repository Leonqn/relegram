pub mod send;
pub mod get;

pub trait Request {
    fn method(&self) -> &'static str;
}