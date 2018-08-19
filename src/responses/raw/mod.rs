pub mod message;
pub mod user;
pub mod chat;
pub mod update;
pub mod queries;
pub mod shipping_address;
pub mod order_info;

#[derive(Deserialize, Debug)]
pub struct TgResponse<T> {
    pub ok: bool,
    pub result: Option<T>,
    pub description: Option<String>,
    pub error_code: Option<i32>
}