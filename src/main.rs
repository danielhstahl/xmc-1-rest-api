mod commands;

use commands::{EmotivaControl, EmotivaPing};
//use serde_xml_rs::{to_string, Serializer};
use quick_xml::se::to_string;
use std::{collections::HashMap, net::UdpSocket};
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
    let result = to_string(&ping).unwrap();

    println!("{}", result);
    let command = EmotivaControl::new("0".to_string());

    // let commandtest = HashMap::from([("myCommand".to_owned(), "do_something".to_string())]);

    //let mut se = Serializer::new_from_reader(in_xml.as_bytes()).non_contiguous_seq_elements(true);

    let result = to_string(&command).unwrap();
    println!("{}", result);

    // <?xml version="1.0" encoding="utf-8"?>
    // <emotivaControl>
    //    <power_on value="0" ack="yes" />
    // </emotivaControl>
}
