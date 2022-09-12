mod pb;
mod storage;
use std::error::Error;
use pb::bokvpb::{CmdRequest, CmdResponse, cmd_request::ReqData, Get, Set};
use anyhow::Result;
use bytes::{BytesMut, Bytes};
use futures::{SinkExt, StreamExt};

use prost::Message;
use tokio::net::TcpStream;
use tokio_util::codec::{Framed, LengthDelimitedCodec};
use tracing::info;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    tracing_subscriber::fmt::init();

    
    

    let stream = TcpStream::connect("127.0.0.1:40889").await?;

    
    let mut stream = Framed::new(stream, LengthDelimitedCodec::new());
    let mut buf = BytesMut::new();

    
    //let cmd_set = CmdRequest::set("mykey", Bytes::from("myvalue"), 0);
    let cmd_get = CmdRequest::get("mykey");
    cmd_get.encode(&mut buf).unwrap();

    
    stream.send(buf.freeze()).await.unwrap();
    info!("Send info successed！");

    
    while let Some(Ok(buf)) = stream.next().await {
        let cmd_res = CmdResponse::decode(&buf[..]).unwrap();
        info!("Receive a response: {:?}", cmd_res);
    }

    Ok(())
}