use requests::Request;

#[derive(Serialize, Debug, Clone)]
pub struct AnswerCallbackQuery {
    pub callback_query_id: String,
    pub text: String,
    pub show_alert: bool,
    pub url: String,
    pub cache_time: i32,
}

impl Request for AnswerCallbackQuery {
    fn method(&self) -> &'static str {
        "answerCallbackQuery"
    }
}