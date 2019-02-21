use requests::Request;
use std::ops::Not;

#[derive(Serialize, Debug, Clone)]
pub struct AnswerCallbackQuery {
    pub callback_query_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    #[serde(skip_serializing_if = "Not::not")]
    pub show_alert: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cache_time: Option<i32>,
}

impl Request for AnswerCallbackQuery {
    fn method(&self) -> &'static str {
        "answerCallbackQuery"
    }
}