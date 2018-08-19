use requests::Request;

#[derive(Serialize, Debug, Clone)]
pub struct GetUpdatesRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<i64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeout: Option<u32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub allowed_updates: Option<Vec<AllowedUpdate>>,
}

#[derive(Serialize, Debug, Clone, Copy)]
#[serde(rename_all = "snake_case")]
pub enum AllowedUpdate {
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

impl Request for GetUpdatesRequest {
    fn method(&self) -> &'static str {
        "getUpdates"
    }
}

impl GetUpdatesRequest {
    pub fn new() -> GetUpdatesRequest {
        GetUpdatesRequest {
            offset: None,
            limit: None,
            timeout: None,
            allowed_updates: None,
        }
    }
}
