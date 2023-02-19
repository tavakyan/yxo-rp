use async_trait::async_trait;
use thiserror::Error;
use crate::common::*;

/// Send a request to an origin and return the response
/// This will block until the response is returned or an error.
#[async_trait]
pub trait OriginSyncRequestPort: Send + Sync {
    async fn send(&self, request: &Request) -> OriginSyncRequestResult<Response>;
}

#[derive(Debug, Error)]
pub enum OriginSyncRequestError {
    #[error("unknown receiver error")]
    Unknown,
}

pub type OriginSyncRequestResult<T> = Result<T, OriginSyncRequestError>;

/// Send an async request to an origin and return the request id.
/// The request id can be used to try to receive a response if it available.
#[async_trait]
pub trait OriginRequestPort: Send + Sync {
    async fn send(&self, request: &Request) -> OriginRequestResult<()>;

    async fn try_receive(&self, id: MessageDigest) -> OriginRequestResult<Option<(RequestDigest, Response)>>;
}

#[derive(Debug, Error)]
pub enum OriginRequestError {
    #[error("unknown receiver error")]
    Unknown,
}

pub type OriginRequestResult<T> = Result<T, OriginRequestError>;