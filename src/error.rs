use hyper;
use serde_json;
use std::fmt;
use std::error;

#[derive(Debug)]
pub enum Error {
    Hyper(hyper::Error),
    Serde(serde_json::Error),
    TelegramApi { error_code: i32, description: String },
    UnexpectedResponse(UnexpectedResponse),
    UnknownError(String),
}


#[derive(Debug)]
pub enum UnexpectedResponse {
    UnexpectedUpdate { id: i32, kind: UnexpectedUpdate }
}

#[derive(Debug)]
pub enum UnexpectedUpdate {
    UnexpectedMessage(UnexpectedMessage),
    Unsupported,
}

#[derive(Debug)]
pub enum UnexpectedMessage {
    /// this should not really happen
    WrongForwardArguments,
    Unsupported,
}

impl From<UnexpectedMessage> for UnexpectedUpdate {
    fn from(x: UnexpectedMessage) -> Self {
        UnexpectedUpdate::UnexpectedMessage(x)
    }
}

impl From<UnexpectedResponse> for Error {
    fn from(x: UnexpectedResponse) -> Self {
        Error::UnexpectedResponse(x)
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
                write!(f, "Hyper error has occured: {}", hyper),

            Error::Serde(serde) =>
                write!(f, "Serde error has occured: {}", serde),

            Error::TelegramApi { error_code, description } =>
                write!(f, "Error response from telegram bot api: error_code: {}, description: {}", error_code, description),

            Error::UnexpectedResponse(unexpected_response) =>
                match unexpected_response {
                    UnexpectedResponse::UnexpectedUpdate { id, kind } =>
                        match kind {
                            UnexpectedUpdate::UnexpectedMessage(unexpected_message) =>
                                match unexpected_message {
                                    UnexpectedMessage::WrongForwardArguments =>
                                        write!(f, "Unexpected forwards fields combination in update {}", id),
                                    UnexpectedMessage::Unsupported =>
                                        write!(f, "Unsupported message in {}", id),
                                }
                            UnexpectedUpdate::Unsupported =>
                                write!(f, "Unsupported update in update {}", id),
                        }
                }

            Error::UnknownError(s) =>
                write!(f, "Unknown error has occured: {}", s)
        }
    }
}