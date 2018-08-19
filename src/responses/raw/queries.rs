use responses::raw::user::User;
use responses::raw::message::Location;
use responses::raw::message::Message;
use responses::raw::shipping_address::ShippingAddress;
use responses::raw::order_info::OrderInfo;

#[derive(Deserialize, Debug, Clone)]
pub struct InlineQuery {
    pub id: String,
    pub from: User,
    pub location: Option<Location>,
    pub query: String,
    pub offset: String
}

#[derive(Deserialize, Debug, Clone)]
pub struct ChosenInlineResult {
    pub result_id: String,
    pub from: User,
    pub location: Option<Location>,
    pub inline_message_id: Option<String>,
    pub query: String
}

#[derive(Deserialize, Debug, Clone)]
pub struct CallbackQuery {
    pub id: String,
    pub from: User,
    pub message: Option<Message>,
    pub inline_message_id: Option<String>,
    pub chat_instance: String,
    pub data: Option<String>,
    pub game_short_name: Option<String>
}

#[derive(Deserialize, Debug, Clone)]
pub struct ShippingQuery {
    pub id: String,
    pub from: User,
    pub invoice_payload: String,
    pub shipping_address: ShippingAddress
}

#[derive(Deserialize, Debug, Clone)]
pub struct PreCheckoutQuery {
    pub id: String,
    pub from: User,
    pub currency: String,
    pub total_amount: i64,
    pub invoice_payload: String,
    pub shipping_option_id: String,
    pub order_info: OrderInfo
}