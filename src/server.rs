

mod pb;
mod storage;
use std::sync::{Arc, Mutex};
use pb::bokvpb::{CmdRequest, CmdResponse, cmd_request::ReqData, Get, Set};
use storage::osfile::OsFileBoKv;

use anyhow::Result;
use bytes::Bytes;
use futures::{SinkExt, StreamExt};

use prost::Message;


use tokio::{net::TcpListener};
use tokio_util::codec::{Framed, LengthDelimitedCodec};
use tracing::info;
#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();
    let path = std::path::Path::new("./bokv.db");
    let mut store = OsFileBoKv::open(path).expect("unable to open file");
    store.load().expect("unable to load data");

    let astore = Arc::new(Mutex::new(store));
    let listener = TcpListener::bind("127.0.0.1:40889").await?;
    loop {
        info!("Waiting for connection...");
        let (stream, addr) = listener.accept().await?;
        info!("Client: {:?} connected", addr);
        let store = astore.clone();
        tokio::spawn(async move {
            
            let mut stream = Framed::new(stream, LengthDelimitedCodec::new());
            while let Some(Ok(mut buf)) = stream.next().await {
                let cmd_req = CmdRequest::decode(&buf[..]).unwrap();
                info!("Receive a command: {:?}", cmd_req);

                buf.clear();

                let cmd_resp = match cmd_req.req_data {
                    Some(ReqData::Get(Get { key })) => {
                        info!("Get key: {}", key);
                        
                        let resp = match store.lock().expect("msg").get(key.as_bytes())
                        {
                            Ok(Some(value)) => {
                                CmdResponse::new(0, "OK".to_string(), Bytes::from(value))
                            }
                            Ok(None) => CmdResponse::new(1, "Key not found".to_string(), Bytes::new()),
                            Err(err) => CmdResponse::new(2, format!("Error: {}", err), Bytes::new()),
                        };
                        resp
                        
                        
                    }
                    Some(ReqData::Set(Set { key, value, expire })) => {
                        info!(
                            "Set key: {}, value: {}, expire: {}",
                            key,
                            value.len(),
                            expire
                        );
                        let resp = match store.lock().expect("msg").insert(key.as_ref(), value.as_ref())
                        {
                            Ok(_) => CmdResponse::new(0, "OK".to_string(), Bytes::new()),
                            Err(err) => CmdResponse::new(2, format!("Error: {}", err), Bytes::new()),
                        };
                        resp
                        
                        
                    }
                    None => {
                        info!("Invalid command");
                        CmdResponse::new(1, "Invalid command".to_string(), Bytes::from(""))
                    }
                };

                cmd_resp.encode(&mut buf).unwrap();
                stream.send(buf.freeze()).await.unwrap();
            }
        });
    }
}
