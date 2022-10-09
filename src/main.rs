mod commands;
use commands::{EmotivaControl, EmotivaPing, EmotivaTransponder};
use quick_xml::de::from_str;
use quick_xml::se::to_string;
use quick_xml::DeError;
use std::env;
use std::error::Error;
use std::io;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::str::from_utf8;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::UdpSocket;
const XMC_PORT: usize = 7000;
const XMC_RECEIVE_PORT: usize = 7001;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let remote_addr: SocketAddr = env::args()
        .nth(1)
        .unwrap_or_else(|| "192.168.4.75:".to_string() + &XMC_PORT.to_string())
        .parse()?;

    // We use port 0 to let the operating system allocate an available port for us.
    let local_addr: SocketAddr = if remote_addr.is_ipv4() {
        "0.0.0.0:".to_string() + &XMC_RECEIVE_PORT.to_string()
    } else {
        "[::]:".to_string() + &XMC_RECEIVE_PORT.to_string()
    }
    .parse()?;

    let socket = UdpSocket::bind(local_addr).await?;
    const MAX_DATAGRAM_SIZE: usize = 65_507;
    socket.connect(&remote_addr).await?;
    let ping = EmotivaPing {};
    let ping_se = to_string(&ping).unwrap();
    socket.send(ping_se.as_bytes()).await?;

    let mut data = vec![0u8; MAX_DATAGRAM_SIZE];
    let len = socket.recv(&mut data).await?;
    let result: Result<EmotivaTransponder, DeError> = from_str(from_utf8(&data[..len]).unwrap());
    println!("{}", result.unwrap().name);

    Ok(())
}
