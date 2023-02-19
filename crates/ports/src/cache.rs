use std::time::Duration;
use async_trait::async_trait;
use thiserror::Error;
use crate::common::*;

/// Generic message store to fetch, insert and delete messages in a buckets.
#[async_trait]
pub trait MessageStorePort: Send + Sync {
    /// Try to insert a message by digest into a bucket.
    /// Returns true on success and false if the message already inserted.
    async fn try_insert(&self, bucket: &MessageBucket, digest: &MessageDigest, message: &Message) -> CacheResult<bool>;

    /// Try to retrieve a message in a bucket given the message digest
    /// Returns some message on success and None if message not in bucket.
    async fn try_fetch(&self, bucket: &MessageBucket, digest: &MessageDigest) -> CacheResult<Option<Message>>;

    /// Check if we have a message in this bucket by digest.
    /// Returns true if the message is in the bucket and false otherwise.
    async fn contains(&self, bucket: &MessageBucket, digest: &MessageDigest) -> CacheResult<bool>;

    /// Clean cache buckets using ttl
    async fn clean(&self, ttl: Duration) -> CacheResult<()>;
}

#[derive(Debug, Error)]
pub enum CacheError {
    #[error("unknown receiver error")]
    Unknown,
}

pub type CacheResult<T> = Result<T, CacheError>;