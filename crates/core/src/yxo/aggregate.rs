
use std::collections::{HashMap, HashSet, VecDeque};
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use cqrs_es::Aggregate;
use yxo_ports::*;

use crate::yxo::*;

/// The ReverseProxyAggregate processes pending requests using a cache and if necessary origin servers.
#[derive(Default, Serialize, Deserialize)]
pub struct ReverseProxyAggregate {
    pending_cache: VecDeque<Request>,
    pending_cache_update: VecDeque<(RequestDigest, Response)>,
    pending_origin: HashMap<RequestDigest, HashSet<Request>>,
    pending_edge: HashMap<Response, HashSet<RequestHeader>>,
}

#[async_trait]
impl Aggregate for ReverseProxyAggregate {
    type Command = ReverseProxyCommand;
    type Event = ReverseProxyEvent;
    type Error = ReverseProxyError;
    type Services = ReverseProxyServices;

    fn aggregate_type() -> String {
        "ReverseProxyAggregate".to_string()
    }

    async fn handle(&self, command: Self::Command, service: &Self::Services) -> Result<Vec<Self::Event>, Self::Error> {
        let mut events = vec![];
        match command {
            ReverseProxyCommand::ReceiveEdgeRequest => {
                let request = service.edge.receive().await.map_err(|_| ReverseProxyError::EdgeError)?;
                let supported = service.is_supported(&request);
                let event = ReverseProxyEvent::EdgeRequestReceived { request, supported };
                Ok(vec![event])
            }
            ReverseProxyCommand::TryRetrieveCacheResponse => {
                let cache_request = self.pending_cache.iter().next();
                if cache_request.is_none() { return Ok(vec![]); }

                let request = cache_request.unwrap();
                let request_digest = service.hasher.digest(request).await.map_err(|_e| ReverseProxyError::HasherError)?;
                let response = service.cache.try_fetch(RESP_BUCKET, &request_digest).await.map_err(|_| ReverseProxyError::CacheError)?;
                let event = match response {
                    None => { ReverseProxyEvent::CacheResponseNotRetrieved { request_digest, request: request.clone() }}
                    Some(response) => { ReverseProxyEvent::CacheResponseRetrieved { request_digest, request: request.clone(), response }}
                };
                Ok(vec![event])
            }
            ReverseProxyCommand::RetrieveOriginResponse  => {
                let origin_request = self.pending_origin.iter().next();
                if origin_request.is_none() { return Ok(vec![]); }

                let (request_digest, requests) = origin_request.unwrap();
                let request = requests.iter().next();
                if request.is_none() { return Ok(vec![]); }

                let request = request.unwrap().clone();
                let response = service.origin.send(&request).await.map_err(|_| ReverseProxyError::OriginError)?;
                let event = ReverseProxyEvent::OriginResponseRetrieved { request_digest: request_digest.clone(), request, response };
                events.push(event);
                Ok(events)
            }
            ReverseProxyCommand::SendEdgeResponse => {
                let edge_response = self.pending_edge.iter().next();
                if edge_response.is_none() { return Ok(vec![]); }

                let (response, headers) = edge_response.unwrap();
                service.edge.send(headers, response).await.map_err(|_| ReverseProxyError::EdgeError)?;
                let event = ReverseProxyEvent::EdgeResponseSent { headers: Vec::from_iter(headers.clone()), response: response.clone() };
                events.push(event);
                Ok(events)
            }
            ReverseProxyCommand::CleanCache { ttl } => {
                service.cache.clean(ttl).await.map_err(|_| ReverseProxyError::CacheError)?;
                let event = ReverseProxyEvent::CacheCleaned {};
                Ok(vec![event])
            }
        }
    }

    fn apply(&mut self, event: Self::Event) {
        match event {
            ReverseProxyEvent::EdgeRequestReceived { request, supported } => {
                if !supported { return; }
                self.pending_cache.push_back(request);
            }
            ReverseProxyEvent::CacheResponseRetrieved { request_digest: _, request, response } => {
                // Update pending edge request headers by response
                self.pending_edge.entry(response)
                    .or_insert_with(|| HashSet::from([request.header.clone()]))
                    .insert(request.header);
            }
            ReverseProxyEvent::CacheResponseNotRetrieved { request_digest, request } => {
                // Add request to pending origin by request digest
                self.pending_origin.entry(request_digest)
                    .or_insert_with(||HashSet::from([request.clone()]))
                    .insert(request);
            }
            ReverseProxyEvent::OriginResponseRetrieved { request_digest, request, response} => {
                // The cache should be updated with this response
                self.pending_cache_update.push_back((request_digest, response.clone()));
                // Update pending edge request headers by response
                self.pending_edge.entry(response)
                    .or_insert_with(|| HashSet::from([request.header.clone()]))
                    .insert(request.header);
            }
            ReverseProxyEvent::EdgeResponseSent { headers: _, response } => {
                self.pending_edge.remove(&response);
            }
            ReverseProxyEvent::CacheCleaned => {}
        }
    }
}