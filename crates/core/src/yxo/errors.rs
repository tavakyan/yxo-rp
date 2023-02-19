use thiserror::Error;

#[derive(Debug, Error)]
pub enum ReverseProxyError {
    #[error("Unable to send or receive client message from edge")]
    EdgeError,
    #[error("Unable to process message digests / hashes")]
    HasherError,
    #[error("Unable to get cache message")]
    CacheError,
    #[error("Unable to communicate with origin")]
    OriginError,
}

pub type ReverseProxyResult<T> = Result<T, ReverseProxyError>;