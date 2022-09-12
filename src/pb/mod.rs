pub mod bokvpb;

use bokvpb::{ cmd_request::ReqData, CmdRequest, CmdResponse, Get, Set};
use bytes::Bytes;




impl CmdRequest {
    // GET
    pub fn get(key: impl Into<String>) -> Self {
        Self {
            req_data: Some(ReqData::Get(Get { key: key.into() })),
        }
    }

    // SET
    pub fn set(key: impl Into<String>, value: Bytes, expire: u32) -> Self {
        Self {
            req_data: Some(ReqData::Set(Set {
                key: key.into(),
                value,
                expire,
            })),
        }
    }

}

impl CmdResponse {
    pub fn new(status: u32, message: String, value: Bytes) -> Self {
        Self {
            status,
            message,
            value,
        }
    }
}