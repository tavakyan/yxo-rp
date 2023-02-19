
use cqrs_es::DomainEvent;
use serde::{Deserialize, Serialize};

use yxo_ports::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ReverseProxyEvent {
    EdgeRequestReceived { request: Request, supported: bool },
    CacheResponseRetrieved { request_digest: RequestDigest, request: Request, response: Response, },
    CacheResponseNotRetrieved { request_digest: RequestDigest, request: Request },
    OriginResponseRetrieved { request_digest: RequestDigest, request: Request, response: Response },
    EdgeResponseSent { headers: Vec<RequestHeader>, response: Response },
    CacheCleaned,
}

impl DomainEvent for ReverseProxyEvent {
    fn event_type(&self) -> String {
        match self {
            ReverseProxyEvent::EdgeRequestReceived { .. } => { "EdgeRequestReceived".to_string() }
            ReverseProxyEvent::CacheResponseRetrieved { .. } => { "CacheResponseRetrieved".to_string() }
            ReverseProxyEvent::CacheResponseNotRetrieved { .. } => { "CacheResponseNotRetrieved".to_string() }
            ReverseProxyEvent::OriginResponseRetrieved { .. } => { "OriginResponseRetrieved".to_string() }
            ReverseProxyEvent::EdgeResponseSent { .. } => { "EdgeResponseSent".to_string() }
            ReverseProxyEvent::CacheCleaned { .. } => { "CacheCleaned".to_string() }
        }
    }

    fn event_version(&self) -> String {
        "1.0".to_string()
    }
}
