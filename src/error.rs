use oagain::OagainError;
use thiserror::Error;

/// Error type for the ETroll utility
#[derive(Error, Debug)]
pub enum ETrollError {
    #[error("An error occurred while deserializing a server response")]
    DeserializationError(#[from] quick_xml::DeError),

    #[error("An authentication error occurred.")]
    OAgain(#[from] OagainError),

    #[error("A parse error occurred in a URL.")]
    UrlParseError(#[from] url::ParseError),
}

pub type Result<T> = std::result::Result<T, ETrollError>;
