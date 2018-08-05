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

const BASE_API_URI: &'static str = "https://api.telegram.org/bot";

pub struct BotClient {
    http_client: Arc<Client<HttpsConnector<HttpConnector>, Body>>,
    token: String,
}

pub enum HttpClient {
    Default,
    Owned(Client<HttpsConnector<HttpConnector>, Body>),
    Arc(Arc<Client<HttpsConnector<HttpConnector>, Body>>),
}

impl BotClient {
    pub fn new(http_client: HttpClient, token: String) -> BotClient {
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

        BotClient {
            http_client,
            token,
        }
    }

    pub fn get_updates(&self, request: GetUpdates) -> impl Future<Item=Vec<Update>, Error=Error> {
        self.send(request, "getUpdates")
            .and_then(|x: Vec<raw::update::Update>|
                x.into_iter()
                    .map(TryFrom::try_from)
                    .collect::<Result<Vec<Update>, UnexpectedResponse>>()
                    .map_err(From::from))

    }

    fn send<TRequest, TResult>(&self, request: TRequest, method: &'static str) -> impl Future<Item=TResult, Error=Error>
        where TRequest: Serialize,
              TResult: DeserializeOwned,
    {
        let uri = format!("{}{}/{}", BASE_API_URI, self.token, method);
        let request =
            Request::post(uri)
                .header("content-type", "application/json")
                .body(Body::from(serde_json::to_string(&request).expect("Error while serializing request")))
                .expect("While creating request an error has occurred");


        self.http_client.request(request)
            .and_then(|r| r.into_body().concat2())
            .then(|body| {
                let response: raw::TgResponse<TResult> = serde_json::from_slice(&body?)?;
                match response {
                    raw::TgResponse { ok: true, result: Some(res), .. } =>
                        Ok(From::from(res)),

                    raw::TgResponse { ok: false, description: Some(description), error_code: Some(error_code), .. } =>
                        Err(Error::TelegramApi { error_code, description }),

                    _ =>
                        Err(Error::UnknownError(String::from("Unknown telegram response")))
                }
            })
    }
}
