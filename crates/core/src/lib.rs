mod yxo;

use std::time::Duration;
use cqrs_es::CqrsFramework;
use cqrs_es::mem_store::MemStore;
use yxo_ports::*;
use crate::yxo::{ReverseProxyAggregate, ReverseProxyCommand, ReverseProxyServices};

static TTL: Duration = Duration::from_secs(30);

// TODO: run shoujld return ReverseProxyResult
pub async fn run(edge: Box<dyn EdgePort>, hasher: Box<dyn HasherPort>, cache: Box<dyn MessageStorePort>, origin: Box<dyn OriginSyncRequestPort>) {
    let services = ReverseProxyServices::new(edge, hasher, cache, origin);
    let store = MemStore::<ReverseProxyAggregate>::default();
    let cqrs = CqrsFramework::new(store, vec![], services);
    let aggregate_id: &str = "rproxy";
    let ttl = TTL;

    loop {
        tokio::select! {
            _ = cqrs.execute(aggregate_id, ReverseProxyCommand::ReceiveEdgeRequest) => {}
            _ = cqrs.execute(aggregate_id, ReverseProxyCommand::TryRetrieveCacheResponse) => {}
            _ = cqrs.execute(aggregate_id, ReverseProxyCommand::RetrieveOriginResponse) => {}
            _ = cqrs.execute(aggregate_id, ReverseProxyCommand::SendEdgeResponse) => {}
            _ = cqrs.execute(aggregate_id, ReverseProxyCommand::CleanCache { ttl }) => {}
        }
    }
}
