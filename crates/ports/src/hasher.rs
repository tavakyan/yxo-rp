use async_trait::async_trait;
use thiserror::Error;
use crate::common::*;

/// Generic hasher to calculate a message digest (aka hash value) from a message preimage
#[async_trait]
pub trait HasherPort: Send + Sync {
    /// Get the digest for this message
    async fn digest(&self, message: &Message) -> HasherResult<MessageDigest>;
}

#[derive(Debug, Error)]
pub enum HasherError {
    #[error("unknown receiver error")]
    Unknown,
}

pub type HasherResult<T> = Result<T, HasherError>;