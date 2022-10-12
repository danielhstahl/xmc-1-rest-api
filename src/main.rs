mod commands;
use commands::{
    EmotivaAck, EmotivaControl, EmotivaPing, EmotivaTransponder, GetInfo, Info, RequestInfo,
};
use quick_xml::de::from_str;
use quick_xml::se::to_string;
use rocket::fs::FileServer;
use serde::Serialize;
use std::env;
use std::error::Error;
use std::io;
use std::net::{IpAddr, Ipv4Addr, Ipv6Addr, SocketAddr};
use std::str::from_utf8;
use tokio::net::UdpSocket;
#[macro_use]
extern crate rocket;
use rocket::serde::json::Json;
use rocket::serde::json::{json, Value};
use rocket::State;

const XMC_PORT: u16 = 7000;
const XMC_RECEIVE_PORT: u16 = 7001;
const MAX_DATAGRAM_SIZE: usize = 65_507;
async fn get_socket(
    remote_addr: &SocketAddr,
    receive_port: u16,
) -> Result<UdpSocket, Box<dyn Error>> {
    let local_addr: SocketAddr = if remote_addr.is_ipv4() {
        SocketAddr::new(IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)), receive_port)
    } else {
        SocketAddr::new(
            IpAddr::V6(Ipv6Addr::new(0, 0, 0, 0, 0, 0, 0, 0)),
            receive_port,
        )
    };
    let socket = UdpSocket::bind(local_addr).await?;
    Ok(socket)
}

async fn get_init_data(
    socket: &UdpSocket,
    init_remote_addr: &SocketAddr,
) -> Result<EmotivaTransponder, Box<dyn Error>> {
    let ping = EmotivaPing {};
    let ping_se = to_string(&ping)?;
    socket
        .send_to(ping_se.as_bytes(), &init_remote_addr)
        .await?;

    let mut data = vec![0u8; MAX_DATAGRAM_SIZE];
    let len = socket.recv(&mut data).await?;
    let transponder: EmotivaTransponder = from_str(from_utf8(&data[..len])?)?;
    Ok(transponder)
}

pub struct SocketState {
    pub socket: UdpSocket,
    pub address: SocketAddr,
}

impl SocketState {
    fn new(socket: UdpSocket, address: SocketAddr) -> Self {
        SocketState { socket, address }
    }
}

async fn send_and_receive_command<S: Serialize>(
    command: S,
    socket_state: &State<SocketState>,
) -> io::Result<String> {
    let serialized_command =
        to_string(&command).map_err(|e| io::Error::new(io::ErrorKind::Interrupted, e))?;
    socket_state
        .socket
        .send_to(serialized_command.as_bytes(), &socket_state.address)
        .await?;
    let mut data = vec![0u8; MAX_DATAGRAM_SIZE];
    let len = socket_state.socket.recv(&mut data).await?;
    let result =
        from_utf8(&data[..len]).map_err(|e| io::Error::new(io::ErrorKind::Interrupted, e))?;
    Ok(result.to_string())
}

#[post("/power/on")]
async fn power(socket_state: &State<SocketState>) -> io::Result<Value> {
    let power = EmotivaControl::power_on();
    let result = send_and_receive_command(power, socket_state).await?;
    println!("this is a result: {}", result);
    let _response: EmotivaAck =
        from_str(&result).map_err(|e| io::Error::new(io::ErrorKind::Interrupted, e))?;
    Ok(json!({ "status": "ok" }))
}

#[post("/power/off")]
async fn standby(socket_state: &State<SocketState>) -> io::Result<Value> {
    let power = EmotivaControl::standby();
    let result = send_and_receive_command(power, socket_state).await?;
    let _response: EmotivaAck =
        from_str(&result).map_err(|e| io::Error::new(io::ErrorKind::Interrupted, e))?;
    Ok(json!({ "status": "ok" }))
}

#[post("/input/<hdmi_input>")]
async fn hdmi_input(hdmi_input: String, socket_state: &State<SocketState>) -> io::Result<Value> {
    let hdmi = match hdmi_input.as_str() {
        "HDMI 1" => Ok(EmotivaControl::set_hdmi_1()),
        "HDMI 2" => Ok(EmotivaControl::set_hdmi_2()),
        "HDMI 3" => Ok(EmotivaControl::set_hdmi_3()),
        "HDMI 4" => Ok(EmotivaControl::set_hdmi_4()),
        "HDMI 5" => Ok(EmotivaControl::set_hdmi_5()),
        "HDMI 6" => Ok(EmotivaControl::set_hdmi_6()),
        "HDMI 7" => Ok(EmotivaControl::set_hdmi_7()),
        "HDMI 8" => Ok(EmotivaControl::set_hdmi_8()),
        _ => Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "Source must be between HDMI 1 and HDMI 8",
        )),
    }?;
    let result = send_and_receive_command(hdmi, socket_state).await?;
    let _response: EmotivaAck =
        from_str(&result).map_err(|e| io::Error::new(io::ErrorKind::Interrupted, e))?;
    Ok(json!({ "status": "ok" }))
}

#[post("/mode/<mode>")]
async fn mode(mode: String, socket_state: &State<SocketState>) -> io::Result<Value> {
    let hdmi = match mode.as_str() {
        "Stereo" => Ok(EmotivaControl::direct()),
        "Dolby" => Ok(EmotivaControl::dolby()),
        "DTS" => Ok(EmotivaControl::dts()),
        "All Stereo" => Ok(EmotivaControl::all_stereo()),
        "Auto" => Ok(EmotivaControl::auto()),
        "Reference Stereo" => Ok(EmotivaControl::reference_stereo()),
        _ => Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "input must be one of Stereo, Dolby, DTS, All Stereo, Auto, Reference Stereo",
        )),
    }?;
    let result = send_and_receive_command(hdmi, socket_state).await?;
    let _response: EmotivaAck =
        from_str(&result).map_err(|e| io::Error::new(io::ErrorKind::Interrupted, e))?;
    Ok(json!({ "status": "ok" }))
}

#[post("/volume/up")]
async fn volume_up(socket_state: &State<SocketState>) -> io::Result<Value> {
    let volume = EmotivaControl::volume_up(1);
    let _response: EmotivaAck = from_str(&send_and_receive_command(volume, socket_state).await?)
        .map_err(|e| io::Error::new(io::ErrorKind::Interrupted, e))?;
    Ok(json!({ "status": "ok" }))
}
#[post("/volume/down")]
async fn volume_down(socket_state: &State<SocketState>) -> io::Result<Value> {
    let volume = EmotivaControl::volume_down(1);
    let _response: EmotivaAck = from_str(&send_and_receive_command(volume, socket_state).await?)
        .map_err(|e| io::Error::new(io::ErrorKind::Interrupted, e))?;
    Ok(json!({ "status": "ok" }))
}
#[post("/volume/<volume>")]
async fn set_volume(volume: i32, socket_state: &State<SocketState>) -> io::Result<Value> {
    if volume < -96 || volume > 11 {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "volume must be between -96 and 11 inclusive",
        ));
    }
    let volume = EmotivaControl::set_volume(volume);
    let _response: EmotivaAck = from_str(&send_and_receive_command(volume, socket_state).await?)
        .map_err(|e| io::Error::new(io::ErrorKind::Interrupted, e))?;
    Ok(json!({ "status": "ok" }))
}

//I just need to periodically ping this to get the latest status... :(.
#[get("/info")]
async fn info(socket_state: &State<SocketState>) -> io::Result<Json<Info>> {
    let info = RequestInfo::new();
    let response: GetInfo = from_str(&send_and_receive_command(info, socket_state).await?)
        .map_err(|e| io::Error::new(io::ErrorKind::Interrupted, e))?;

    Ok(Json(Info::from(response)))
}

#[rocket::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let remote_ip: IpAddr = env::args()
        .nth(1)
        .unwrap_or_else(|| "192.168.4.75".to_string())
        .parse()?;
    let init_remote_addr = SocketAddr::new(remote_ip, XMC_PORT);
    let socket = get_socket(&init_remote_addr, XMC_RECEIVE_PORT).await?;
    let transponder = get_init_data(&socket, &init_remote_addr).await?;
    let remote_addr = SocketAddr::new(remote_ip, transponder.control.control_port);
    let socket = get_socket(&init_remote_addr, transponder.control.control_port).await?;
    let socket_state = SocketState::new(socket, remote_addr);
    let _rocket = rocket::build()
        .manage(socket_state)
        .mount("/", FileServer::from("./xmc-remote/build"))
        .mount(
            "/",
            routes![
                power,
                standby,
                hdmi_input,
                mode,
                info,
                volume_up,
                volume_down,
                set_volume
            ],
        )
        .ignite()
        .await?
        .launch()
        .await?;

    Ok(())
}
