use responses::raw::user::User;
use responses::raw::chat::Chat;

#[derive(Deserialize, Debug)]
pub struct Message {
    pub message_id: i32,
    pub from: Option<User>,
    pub date: i32,
    pub chat: Chat,
    pub forward_from: Option<User>,
    pub forward_from_chat: Option<Chat>,
    pub forward_from_message_id: Option<i32>,
    pub forward_signature: Option<String>,
    pub forward_date: Option<i32>,
    pub reply_to_message: Option<Box<Message>>,
    pub edit_date: Option<i32>,
    pub media_group_id: Option<String>,
    pub author_signature: Option<String>,
    pub text: Option<String>,
    //    pub entities: Option<Vec<MessageEntity>>,
//    pub caption_entities: Option<Vec<MessageEntity>>,
//    pub audio: Option<Audio>,
//    pub document: Option<Document>,
//    pub animation: Option<Animation>,
//    pub game: Option<Game>,
//    pub photo: Option<Vec<Photo>>,
//    pub sticker: Option<Stricker>,
//    pub video: Option<Video>,
    pub voice: Option<Voice>,
//    pub video_note: Option<VideoNote>,
//    pub caption: Option<String>,
//    pub contact: Option<Contact>,
//    pub location: Option<Location>,
//    pub venue: Option<Venue>,
//    pub new_chat_members: Option<Vec<User>>,
//    pub left_chat_member: Option<User>,
//    pub new_chat_title: Option<String>,
//    pub new_chat_photo: Option<Vec<String>>,
//    pub delete_chat_photo: Option<bool>,
//    pub group_chat_created: Option<bool>,
//    pub supergroup_chat_created: Option<bool>,
//    pub channel_chat_created: Option<bool>,
//    pub migrate_to_chat_id: Option<i32>,
//    pub migrate_from_chat_id: Option<i32>,
//    pub pinned_message: Option<Message>,
//    pub invoice: Option<Invoice>,
//    pub successful_payment: Option<SuccessfulPayment>,
//    pub connected_website: Option<string>,
//    pub passport_data: Option<PassportData>
}


#[derive(Deserialize, Debug)]
pub struct Voice {
    pub file_id: String,
    pub duration: i32,
    pub mime_type: Option<String>,
    pub file_size: Option<i32>
}

#[derive(Deserialize, Debug)]
pub struct File {
    pub file_id: String,
    pub file_size: Option<i32>,
    pub file_path: Option<String>

}