use yxo_ports::{MessageStorePort, EdgePort, OriginSyncRequestPort, Request, HasherPort};

pub static RESP_BUCKET: &str = "RESP";
static CAT_FACT: &str = "https://catfact.ninja/fact";
static COIN_DESK: &str = "https://api.coindesk.com/v1/bpi/currentprice.json";
static ORIGINS: [&str; 2] = [CAT_FACT, COIN_DESK];


pub struct ReverseProxyServices {
    pub edge: Box<dyn EdgePort>,
    pub hasher: Box<dyn HasherPort>,
    pub cache: Box<dyn MessageStorePort>,
    pub origin: Box<dyn OriginSyncRequestPort>,
}

impl ReverseProxyServices {
    pub (crate) fn new(edge: Box<dyn EdgePort>, hasher: Box<dyn HasherPort>, cache: Box<dyn MessageStorePort>, origin: Box<dyn OriginSyncRequestPort>) -> Self {
        Self {
            edge,
            hasher,
            cache,
            origin
        }
    }

    /// Determine if a request's origin destination is supported
    pub(crate) fn is_supported(&self, request: &Request) -> bool {
        let url = request.header.dst.as_str();
        ORIGINS.contains(&url)
    }
}