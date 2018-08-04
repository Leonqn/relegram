use hyper;
use serde_json;
use std::fmt;
use std::error;

#[derive(Debug)]
pub enum Error {
    Hyper(hyper::Error),
    Serde(serde_json::Error),
    Telegram { error_code: i32, description: String },
    BadMessage(BadMessage),
    UnknownError(String),
}

#[derive(Debug)]
pub enum BadMessage {
    WrongForwardArguments
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
                write!(f, "Hyper error has occured: {}", hyper),

            Error::Serde(serde) =>
                write!(f, "Serde error has occured: {}", serde),

            Error::Telegram {error_code, description} =>
                write!(f, "Error response from telegram bot api: error_code: {}, description: {}", error_code, description),

            Error::BadMessage(message) =>
                match message {
                    BadMessage::WrongForwardArguments =>
                        write!(f, "Unexpected message forwards field combination"),
                }

            Error::UnknownError(s) =>
                write!(f, "Unknown error has occured: {}", s)
        }

    }
}