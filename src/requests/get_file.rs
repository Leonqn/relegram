use requests::Request;

#[derive(Serialize, Debug, Clone)]
pub struct GetFileRequest {
    pub file_id: String
}

impl Request for GetFileRequest {
    fn method(&self) -> &'static str {
        "getFile"
    }
}