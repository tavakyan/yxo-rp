use std::time::{Duration};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum ReverseProxyCommand {
    ReceiveEdgeRequest,
    TryRetrieveCacheResponse,
    RetrieveOriginResponse,
    SendEdgeResponse,
    CleanCache { ttl: Duration }
}
