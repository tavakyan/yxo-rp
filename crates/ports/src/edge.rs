use std::collections::HashSet;
use thiserror::Error;
use crate::common::*;

/// Generic port to receive requests from clients to be forwarded to origins and send responses back
#[async_trait::async_trait]
pub trait EdgePort: Send + Sync {
    async fn receive(&self) -> EdgeResult<Request>;

    async fn send(&self, header: &HashSet<ResponseHeader>, response: &Response) -> EdgeResult<()>;
}

#[derive(Debug, Error)]
pub enum EdgeError {
    #[error("unknown receiver error")]
    Unknown,
}

pub type EdgeResult<T> = Result<T, EdgeError>;