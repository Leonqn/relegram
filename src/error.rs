use hyper;
use serde_json;
use std::fmt;
use std::error;
use tokio;
use std::time::Duration;

#[derive(Debug)]
pub enum Error {
    Hyper(hyper::Error),
    TokioTimer(tokio::timer::Error),
    TimedOut(Duration),
    Serde(serde_json::Error),
    TelegramApi { error_code: i32, description: String },
    UnexpectedResponse { raw_response: String, kind: UnexpectedResponse },
    UnknownError(String),
}


#[derive(Debug)]
pub enum UnexpectedResponse {
    ConvertError(String),
    Unsupported,
}

impl From<tokio::timer::Error> for Error {
    fn from(err: tokio::timer::Error) -> Self {
        Error::TokioTimer(err)
    }
}

impl From<hyper::Error> for Error {
    fn from(err: hyper::Error) -> Self {
        Error::Hyper(err)
    }
}

impl From<serde_json::Error> for Error {
    fn from(err: serde_json::Error) -> Self {
        Error::Serde(err)
    }
}

impl error::Error for Error {
    fn cause(&self) -> Option<&error::Error> {
        match self {
            Error::Hyper(hyper) =>
                Some(hyper),

            Error::Serde(serde) =>
                Some(serde),

            _ =>
                None
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::Hyper(hyper) =>
                write!(f, "Hyper error has occurred: {}", hyper),
            Error::TokioTimer(tokio) =>
                write!(f, "Tokio timer error has occurred: {}", tokio),
            Error::TimedOut(timeout) =>
                write!(f, "Request timed out. Provided budget: {} seconds", timeout.as_secs()),
            Error::Serde(serde) =>
                write!(f, "Serde error has occurred: {}", serde),

            Error::TelegramApi { error_code, description } =>
                write!(f, "Error response from telegram bot api: error_code: {}, description: {}", error_code, description),

            Error::UnexpectedResponse { raw_response, kind } =>
                match kind {
                    UnexpectedResponse::ConvertError(s) =>
                        write!(f, "Convert from raw data to data model. {}. See raw_response:  {}", s, raw_response),
                    UnexpectedResponse::Unsupported =>
                        write!(f, "Unsupported response. See raw_response: {}", raw_response),
                }

            Error::UnknownError(s) =>
                write!(f, "Unknown error has occurred: {}", s)
        }
    }
}