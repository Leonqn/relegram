use responses::raw::message::Message;

pub mod message;
pub mod user;
pub mod chat;
pub mod update;

#[derive(Deserialize, Debug)]
pub struct TgResponse<T> {
    pub ok: bool,
    pub result: Option<T>,
    pub description: Option<String>,
    pub error_code: Option<i32>
}