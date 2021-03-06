use std::sync::Arc;
use hyper::Client;
use hyper_tls::HttpsConnector;
use hyper::client::HttpConnector;
use hyper::Body;
use error::*;
use hyper::rt::{Future, Stream};
use serde::Serialize;
use serde::de::DeserializeOwned;
use hyper::Request;
use serde_json;
use responses::*;
use requests::*;
use std::str;
use stream::UpdatesStream;
use std::collections::VecDeque;
use std::time::Duration;
use std::time::Instant;
use tokio::timer::Delay;
use try_from::TryFrom;
use std::cmp::max;

const BASE_API_URI: &'static str = "https://api.telegram.org/bot";
const GET_FILE_URI: &'static str = "https://api.telegram.org/file/bot";

pub struct BotApiClient {
    http_client: Arc<Client<HttpsConnector<HttpConnector>, Body>>,
    token: Arc<String>,
}

impl Clone for BotApiClient {
    fn clone(&self) -> Self {
        BotApiClient {
            http_client: Arc::clone(&self.http_client),
            token: Arc::clone(&self.token),
        }
    }
}

pub enum HttpClient {
    Default,
    Owned(Client<HttpsConnector<HttpConnector>, Body>),
    Arc(Arc<Client<HttpsConnector<HttpConnector>, Body>>),
}

impl BotApiClient {
    pub fn new<S: Into<String>>(http_client: HttpClient, token: S) -> BotApiClient {
        let http_client =
            match http_client {
                HttpClient::Default => {
                    let https = HttpsConnector::new(1).expect("TLS initialization failed");
                    Arc::new(Client::builder().build::<_, Body>(https))
                }
                HttpClient::Owned(http_client) => {
                    Arc::new(http_client)
                }
                HttpClient::Arc(http_client) => {
                    http_client
                }
            };
        BotApiClient {
            http_client,
            token: Arc::new(token.into()),
        }
    }

    pub fn incoming_updates(&self, mut request: GetUpdatesRequest) -> impl Stream<Item=Update, Error=Error> {
        let cloned_self = self.clone();
        let first_request = cloned_self.get_updates(&request);
        let offset = request.offset;
        let send_request = move |x| {
            request.offset = x;
            cloned_self.get_updates(&request)
        };
        UpdatesStream {
            bot_api_client: send_request,
            buffer: VecDeque::new(),
            executing_request: first_request,
            is_canceled: false,
            last_id: offset,
            has_error: false
        }
    }

    pub fn download_file(&self, request: &GetFileRequest, timeout: Duration) -> impl Future<Item=Vec<u8>, Error=Error> {
        let cloned_self = self.clone();
        let download_future =
            self.get_file(request, timeout)
                .then(|file| {
                    match file? {
                        File { file_path: Some(path), .. } =>
                            Ok(path),
                        _ =>
                            Err(Error::Unknown(String::from("File not found")))
                    }
                })
                .and_then(move |file_path| {
                    let uri = format!("{}{}/{}", GET_FILE_URI, cloned_self.token, file_path).parse().expect("Error has occurred while creating get_file uri");
                    cloned_self.http_client.get(uri)
                        .and_then(|response| {
                            response
                                .into_body()
                                .concat2()
                        })
                        .map(|x| x.to_vec())
                        .map_err(From::from)
                });
        BotApiClient::with_timeout(download_future, timeout)
    }

    pub fn send_message(&self, request: &SendMessageRequest, timeout: Duration) -> impl Future<Item=Message, Error=Error> {
        self.send_request(request, <Message as TryFrom<raw::message::Message>>::try_from, timeout)
    }

    pub fn send_chat_action(&self, request: &SendChatAction, timeout: Duration) -> impl Future<Item=bool, Error=Error> {
        fn id(val: bool) -> Result<bool, UnexpectedResponse> {
            Ok(val)
        }
        self.send_request(request, id, timeout)
    }

    pub fn answer_callback_query(&self, request: &AnswerCallbackQuery, timeout: Duration) -> impl Future<Item=bool, Error=Error> {
        fn id(val: bool) -> Result<bool, UnexpectedResponse> {
            Ok(val)
        }
        self.send_request(request, id, timeout)
    }

    pub fn send_media_group(&self, request: &SendMediaGroupRequest, timeout: Duration) -> impl Future<Item=Vec<Message>, Error=Error> {
        fn map(x: Vec<raw::message::Message>) -> Result<Vec<Message>, UnexpectedResponse> {
            x.into_iter().map(TryFrom::try_from).collect()
        }
        self.send_request(request, map, timeout)
    }

    pub fn get_me(&self, timeout: Duration) -> impl Future<Item=User, Error=Error> {
        self.send_request(&GetMe, Ok, timeout)
    }

    pub fn get_file(&self, request: &GetFileRequest, timeout: Duration) -> impl Future<Item=File, Error=Error> {
        self.send_request(request, Ok, timeout)
    }

    pub fn get_updates(&self, request: &GetUpdatesRequest) -> impl Future<Item=Vec<Update>, Error=Error> {
        fn map(x: Vec<raw::update::Update>) -> Result<Vec<Update>, UnexpectedResponse> {
            x.into_iter()
                .map(TryFrom::try_from)
                .collect::<Result<Vec<Update>, UnexpectedResponse>>()
        }

        let timeout = Duration::from_secs(request.timeout.map(|x| max(x * 2, 10)).unwrap_or(10) as u64);

        self.send_request(request, map, timeout)
    }

    fn send_request<TRequest, TResult, TMappedResult>(&self, request: &TRequest, result_map: fn(TResult) -> Result<TMappedResult, UnexpectedResponse>, timeout: Duration) -> impl Future<Item=TMappedResult, Error=Error>
        where TRequest: Serialize + ::requests::Request,
              TResult: DeserializeOwned,
    {
        let uri = format!("{}{}/{}", BASE_API_URI, self.token, request.method());
        let request =
            Request::post(uri)
                .header("content-type", "application/json")
                .body(Body::from(serde_json::to_string(request).expect("Error while serializing request")))
                .expect("While creating request an error has occurred");


        let api_request = self.http_client.request(request)
            .and_then(|r| r.into_body().concat2())
            .then(move |body| {
                let body_ref = &body?;
                let response: raw::TgResponse<TResult> = serde_json::from_slice(body_ref)?;
                match response {
                    raw::TgResponse { ok: true, result: Some(res), .. } =>
                        result_map(res)
                            .map_err(|err|
                                str::from_utf8(body_ref)
                                    .map(|x| Error::UnexpectedResponse { raw_response: String::from(x), kind: err })
                                    .unwrap_or(Error::Unknown(String::from("Error while converting tg response to utf8 string")))),

                    raw::TgResponse { ok: false, description: Some(description), error_code: Some(error_code), .. } =>
                        Err(Error::TelegramApi { error_code, description }),

                    _ =>
                        Err(str::from_utf8(body_ref)
                            .map(|x| Error::Unknown(String::from(x)))
                            .unwrap_or(Error::Unknown(String::from("Error while converting tg response to utf8 string"))))
                }
            });
        BotApiClient::with_timeout(api_request, timeout)
    }

    fn with_timeout<T>(fut: impl Future<Item=T, Error=Error>, timeout: Duration) -> impl Future<Item=T, Error=Error> {
        let when = Instant::now() + timeout;
        let delay = Delay::new(when)
            .then(move |x| {
                match x {
                    Ok(_) =>
                        Err(Error::TimedOut(timeout)),
                    Err(e) =>
                        Err(From::from(e))
                }
            });
        fut.select(delay).then(|x| {
            match x {
                Ok((resp, _)) =>
                    Ok(resp),
                Err((err, _)) =>
                    Err(err)
            }
        })
    }
}
