use serde::{Serialize, Deserialize};
use bytes::Bytes;

#[derive(Debug, Clone, Serialize, Deserialize, Hash, PartialEq, Eq)]
pub struct MessageHeader {
    pub src: SourceUrl,
    pub dst: DestinationUrl,
}

#[derive(Debug, Clone, Serialize, Deserialize, Hash, PartialEq, Eq)]
pub struct MessageBody {
    pub data: Bytes,
}

#[derive(Debug, Clone, Serialize, Deserialize, Hash, PartialEq, Eq)]
pub struct Message {
    pub header: MessageHeader,
    pub body: MessageBody
}

pub type MessageBucket = str;
pub type Url = String;
pub type SourceUrl = Url;
pub type DestinationUrl = Url;
pub type MessageDigest = Bytes;
pub type RequestDigest = MessageDigest;
pub type ResponseDigest = MessageDigest;
pub type Request = Message;
pub type RequestHeader = MessageHeader;
pub type Response = Message;
pub type ResponseHeader = MessageHeader;