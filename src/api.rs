use std::sync::Arc;
use hyper::Client;
use hyper_tls::HttpsConnector;
use hyper::client::HttpConnector;
use hyper::Body;
use requests::*;
use error::*;
use hyper::rt::{Future, Stream};
use serde::Serialize;
use serde::de::DeserializeOwned;
use hyper::Request;
use serde_json;
use responses::raw;
use responses::update::*;
use std::convert::TryFrom;
use std::str;
use stream::UpdatesStream;
use std::collections::VecDeque;
use std::time::Duration;

const BASE_API_URI: &'static str = "https://api.telegram.org/bot";

pub struct BotApiClient {
    http_client: Arc<Client<HttpsConnector<HttpConnector>, Body>>,
    token: Arc<String>,
}

impl Clone for BotApiClient {
    fn clone(&self) -> Self {
        BotApiClient {
            http_client: Arc::clone(&self.http_client),
            token: Arc::clone(&self.token)
        }
    }
}

pub enum HttpClient {
    Default,
    Owned(Client<HttpsConnector<HttpConnector>, Body>),
    Arc(Arc<Client<HttpsConnector<HttpConnector>, Body>>),
}

impl BotApiClient {
    pub fn new(http_client: HttpClient, token: String) -> BotApiClient {
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
            token: Arc::new(token),
        }
    }

    pub fn incoming_updates(&self, mut request: GetUpdates, timeout: Duration) -> impl Stream<Item=Update, Error=Error> {
        let cloned_self = self.clone();
        let first_request = cloned_self.get_updates(&request, timeout);
        let send_request = move |x| {
            request.offset = x;
            cloned_self.get_updates(&request, timeout) };
        UpdatesStream {
            bot_api_client: send_request,
            buffer: VecDeque::new(),
            executing_request: first_request,
        }
    }

    pub fn get_updates(&self, request: &GetUpdates, timeout: Duration) -> impl Future<Item=Vec<Update>, Error=Error> {
        fn map(x: Vec<raw::update::Update>) -> Result<Vec<Update>, UnexpectedResponse> {
            x.into_iter()
                .map(TryFrom::try_from)
                .collect::<Result<Vec<Update>, UnexpectedResponse>>()
        }

        self.send(request, "getUpdates", map)
    }

    fn send<TRequest, TResult, TMappedResult>(&self, request: &TRequest, method: &'static str, result_map: fn(TResult) -> Result<TMappedResult, UnexpectedResponse>) -> impl Future<Item=TMappedResult, Error=Error>
        where TRequest: Serialize,
              TResult: DeserializeOwned,
    {
        let uri = format!("{}{}/{}", BASE_API_URI, self.token, method);
        let request =
            Request::post(uri)
                .header("content-type", "application/json")
                .body(Body::from(serde_json::to_string(request).expect("Error while serializing request")))
                .expect("While creating request an error has occurred");


        self.http_client.request(request)
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
                                    .unwrap_or(Error::UnknownError(String::from("Error while converting tg response to utf8 string")))),

                    raw::TgResponse { ok: false, description: Some(description), error_code: Some(error_code), .. } =>
                        Err(Error::TelegramApi { error_code, description }),

                    _ =>
                        Err(str::from_utf8(body_ref)
                            .map(|x| Error::UnknownError(String::from(x)))
                            .unwrap_or(Error::UnknownError(String::from("Error while converting tg response to utf8 string"))))
                }
            })
    }
}
