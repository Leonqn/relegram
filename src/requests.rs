#[derive(Serialize, Debug, Default, Clone)]
pub struct GetUpdates {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<i64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeout: Option<i64>,

    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub allowed_updates: Vec<AllowedUpdates>,
}

#[derive(Serialize, Debug, Default, Clone)]
pub struct SendMessage {

}

#[derive(Serialize, Debug, Clone, Copy)]
#[serde(rename_all = "snake_case")]
pub enum AllowedUpdates {
    Message,
    EditedMessage,
    ChannelPost,
    EditedChannelPost,
    InlineQuery,
    ChosenInlineResult,
    CallbackQuery,
    ShippingQuery,
    PreCheckoutQuery,
}