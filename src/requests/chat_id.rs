
#[derive(Serialize, Debug, Clone)]
#[serde(untagged)]
pub enum ChatId {
    Id(i64),
    Username(String),
}
