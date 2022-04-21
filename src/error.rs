use std::fmt;

#[derive(Debug)]
pub enum Error {
    HttpError(reqwest::Error),
    Url(url::ParseError),
}

pub type Result<T> = std::result::Result<T, Error>;

impl std::error::Error for Error {}

impl From<reqwest::Error> for Error {
    fn from(err: reqwest::Error) -> Self {
        Error::HttpError(err)
    }
}

impl From<url::ParseError> for Error {
    fn from(error: url::ParseError) -> Self {
        Error::Url(error)
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::HttpError(e) => write!(f, "{}", e),
            Error::Url(e) => write!(f, "{}", e),
        }
    }
}
