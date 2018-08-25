# relegram
Telegram bot api client.
Under development.

## Example
Example bot that will resend all text and voice messages to sender.

``` rust
extern crate relegram;
extern crate hyper;

use relegram::api::{BotApiClient, HttpClient};
use hyper::rt::{Future, Stream};
use std::time::Duration;
use relegram::requests::*;
use relegram::responses::*;

fn main() {
    let bot_client = BotApiClient::new(HttpClient::Default, String::from("YOUR TOKEN"));
    let get_updates =
        GetUpdatesRequest {
            timeout: Some(20),
            ..GetUpdatesRequest::new()
        };
    let updates = bot_client.incoming_updates(get_updates, Duration::from_secs(30))
        .for_each(move |x| {
            match x.kind {
                UpdateKind::Message(Message { from: MessageFrom::User { chat, .. }, kind: msg, .. }) => {
                    let send =
                        match msg {
                            MessageKind::Text { text, .. } =>
                                SendMessageRequest::new(ChatId::Id(chat.id), SendMessageKind::Text(SendText::new(text))),
                            MessageKind::Voice { voice, .. } =>
                                SendMessageRequest::new(ChatId::Id(chat.id), SendMessageKind::Voice(SendVoice::new(FileKind::FileId(voice.file_id)))),
                            _ =>
                                return Ok(())
                        };
                    hyper::rt::spawn(
                        bot_client.send_message(&send, Duration::from_secs(10))
                            .map(|x| println!("message sent {:?}", x))
                            .map_err(|x| println!("error occurred {:?}", x)));
                }
                _ =>
                return Ok(())

            };
            Ok(())
        })
        .map_err(|x| println!("error {:?}", x));

    hyper::rt::run(updates);
}
```