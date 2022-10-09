//#![feature(const_extern_fn)]

mod commands;
use commands::{EmotivaControl, EmotivaPing, EmotivaTransponder};
use local_ip_address::list_afinet_netifas;
use local_ip_address::local_ip;
use quick_xml::de::from_str;
use quick_xml::se::to_string;
use quick_xml::DeError;
use std::str::from_utf8;
//use std::net::UdpSocket;
use std::net::{IpAddr, Ipv4Addr, UdpSocket};
const XMC_PORT: usize = 7000;
const XMC_RECEIVE_PORT: usize = 7001;
fn main() {
    /*{
        let socket = UdpSocket::bind("127.0.0.1:34254")?;

        // Receives a single datagram message on the socket. If `buf` is too small to hold
        // the message, it will be cut off.
        let mut buf = [0; 10];
        let (amt, src) = socket.recv_from(&mut buf)?;

        // Redeclare `buf` as slice of the received data and send reverse data back to origin.
        let buf = &mut buf[..amt];
        buf.reverse();
        socket.send_to(buf, &src)?;
    } // the socket is closed here*/
    let ping = EmotivaPing {};
    let ping_se = to_string(&ping).unwrap();

    //println!("{}", result);
    let command = EmotivaControl::set_hdmi_1();

    // let commandtest = HashMap::from([("myCommand".to_owned(), "do_something".to_string())]);

    //let mut se = Serializer::new_from_reader(in_xml.as_bytes()).non_contiguous_seq_elements(true);

    let result = to_string(&command).unwrap();
    println!("{}", result);

    let my_local_ip = local_ip().unwrap();
    println!("This is my local IP address: {:?}", my_local_ip);
    let my_ipv4 = match my_local_ip {
        IpAddr::V4(result) => result,
        _ => Ipv4Addr::new(127, 0, 0, 1), //should never get here...I should return a result instead
    };

    let local_upd_address = my_local_ip.to_string() + ":" + &XMC_RECEIVE_PORT.to_string();
    let socket = UdpSocket::bind(local_upd_address).unwrap();
    let [a, b, c, d] = my_ipv4.octets();
    let mut init_d: u8 = 75;
    loop {
        let possible_address =
            Ipv4Addr::new(a, b, c, init_d).to_string() + ":" + &XMC_PORT.to_string();
        let copy_of_possible_address = possible_address.clone();
        socket.connect(possible_address).unwrap();
        socket.send(ping_se.as_bytes()).unwrap();
        //loop {
        let mut buf = [0; 100];
        let num_bytes = socket.recv(&mut buf);
        println!("Attempting ip {}", copy_of_possible_address);
        match num_bytes {
            Ok(message_size) => {
                let possible_result: Result<EmotivaTransponder, DeError> =
                    from_str(from_utf8(&buf[..message_size]).unwrap());
                match possible_result {
                    Ok(result) => {
                        println!("Found on ip {}", copy_of_possible_address);
                    }
                    Err(e) => {
                        init_d += 1;
                    }
                }
            }
            Err(e) => {
                init_d += 1;
            }
        };

        //}
        //let socket = UdpSocket::bind(possible_address);

        /*match socket {
            Ok(socket) => {
                println!("address is {:?}", socket.local_addr());
                socket.send(ping_se.as_bytes());
            }
            Err(e) => {
                println!("still searching");
                init_d += 1;
            }
        }*/

        // Receives a single datagram message on the socket. If `buf` is too small to hold
        // the message, it will be cut off.
        //let mut buf = [0; 10];
        //let (amt, src) = socket.recv_from(&mut buf).unwrap();

        // Redeclare `buf` as slice of the received data and send reverse data back to origin.
        //let buf = &mut buf[..amt];
        //buf.reverse();
        //socket.send_to(buf, &src).unwrap();
    }
    //println!("This is my local IP address: {:?}", my_local_ip);
    // <?xml version="1.0" encoding="utf-8"?>
    // <emotivaControl>
    //    <power_on value="0" ack="yes" />
    // </emotivaControl>
}
