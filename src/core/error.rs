use core::fmt;

#[derive(Debug)]
pub enum AcceloError {
    HttpError,
    UnauthorizedError,
    ParseError,
    RequestError,
}

impl std::error::Error for AcceloError {}

impl fmt::Display for AcceloError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AcceloError::HttpError => write!(f, "Http Error"),
            AcceloError::UnauthorizedError => write!(f, "Unauthorized Error"),
            AcceloError::ParseError => write!(f, "Parse Error"),
            AcceloError::RequestError => write!(f, "Request Error"),
        }
    }
}
