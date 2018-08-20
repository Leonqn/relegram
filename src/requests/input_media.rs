#[derive(Serialize, Debug, Clone)]
pub struct InputMediaPhoto {}

#[derive(Serialize, Debug, Clone)]
pub struct InputMediaVideo {}

#[derive(Serialize, Debug, Clone)]
pub struct InputMediaAnimation {}

#[derive(Serialize, Debug, Clone)]
pub struct InputMediaDocument {}

#[derive(Serialize, Debug, Clone)]
pub struct InputMediaAudio {}

pub enum InputMedia {
    Animation(InputMediaAnimation),
    Document(InputMediaDocument),
    Audio(InputMediaAudio),
    Photo(InputMediaPhoto),
    Video(InputMediaVideo),
}