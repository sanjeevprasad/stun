use webrtc_stun as stun;

use stun::client::*;
use stun::message::*;
use stun::xoraddr::*;
use stun::{agent::*, attributes::*};

use std::sync::Arc;
use tokio::net::UdpSocket;
use webrtc_util::Error;

#[tokio::main]
async fn main() -> Result<(), Error> {
    // let server = "stun.l.google.com:19302";
    let server = "atanx.com:5349";

    println!("Connecting {}...", server);

    let (handler_tx, mut handler_rx) = tokio::sync::mpsc::unbounded_channel();

    let conn = UdpSocket::bind("::0:0").await?;
    println!("local address : {}", conn.local_addr()?);

    conn.connect(server).await?;

    let mut client = ClientBuilder::new().with_conn(Arc::new(conn)).build()?;
    let mut msg = Message::new();
    msg.add(ATTR_USE_CANDIDATE, "abc".as_bytes());
    msg.build(&[
        Box::new(TransactionId::default()),
        Box::new(BINDING_REQUEST),
    ])?;

    client.send(&msg, Some(Arc::new(handler_tx))).await?;

    while let Some(event) = handler_rx.recv().await {
        match event.event_body {
            Ok(msg) => {
                let msg: Message = msg;
                println!("msg.typ            : {}",msg.typ);
                println!("msg.length         : {}", msg.length);
                println!("msg.transaction_id : {:?}", msg.transaction_id.0);
                for attribute in &msg.attributes.0 {
                    println!("attributes         : {}", attribute);
                }
                println!("-------------------------------------------");
                let mut xor_addr = XORMappedAddress::default();
                xor_addr.get_from(&msg)?;
                println!("public address: {}", xor_addr);
            }
            Err(err) => println!("{:?}", err),
        };
    }

    client.close().await?;

    Ok(())
}
