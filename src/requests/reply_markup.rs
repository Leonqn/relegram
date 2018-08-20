use std::ops::Not;

#[derive(Serialize, Debug, Clone)]
#[serde(untagged)]
pub enum ReplyMarkup {
    InlineKeyboard(InlineKeyboard),
    ReplyKeyboardMarkup(ReplyKeyboardMarkup),
    ReplyKeyboardRemove(ReplyKeyboardRemove),
    ForceReply(ForceReply),
}


#[derive(Serialize, Debug, Clone)]
pub struct InlineKeyboard {
    pub inline_keyboard: Vec<Vec<InlineKeyboardButton>>
}

#[derive(Serialize, Debug, Clone)]
pub struct ReplyKeyboardMarkup {
    pub keyboard: Vec<Vec<KeyboardButton>>,
    #[serde(skip_serializing_if = "Not::not")]
    pub resize_keyboard: bool,
    #[serde(skip_serializing_if = "Not::not")]
    pub one_time_keyboard: bool,
    #[serde(skip_serializing_if = "Not::not")]
    pub selective: bool,
}

#[derive(Serialize, Debug, Clone)]
pub struct ReplyKeyboardRemove {
    #[serde(skip_serializing_if = "Not::not")]
    pub remove_keyboard: bool,
    #[serde(skip_serializing_if = "Not::not")]
    pub selective: bool,
}

#[derive(Serialize, Debug, Clone)]
pub struct ForceReply {
    #[serde(skip_serializing_if = "Not::not")]
    pub force_reply: bool,
    #[serde(skip_serializing_if = "Not::not")]
    pub selective: bool,
}

#[derive(Serialize, Debug, Clone)]
pub struct InlineKeyboardButton {
    pub text: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub callback_data: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub switch_inline_query: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub switch_inline_query_current_chat: Option<String>,
    //    pub callback_game: Option<CallbackGame>
    #[serde(skip_serializing_if = "Not::not")]
    pub pay: bool,
}

#[derive(Serialize, Debug, Clone)]
pub struct KeyboardButton {
    pub text: String,
    #[serde(skip_serializing_if = "Not::not")]
    pub request_contact: bool,
    #[serde(skip_serializing_if = "Not::not")]
    pub request_location: bool,
}