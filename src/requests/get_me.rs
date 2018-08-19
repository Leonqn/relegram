use requests::Request;

#[derive(Serialize, Debug, Clone)]
pub struct GetMe;

impl Request for GetMe {
    fn method(&self) -> &'static str {
        "getMe"
    }
}