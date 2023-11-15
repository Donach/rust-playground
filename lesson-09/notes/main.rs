fn main() {
    //main_udp();
    main_ser();
    println!("Hello, world!");
    use std::io::{Read, Write};
    use std::net::{TcpListener, TcpStream};

    /* let mut stream = TcpStream::connect("127.0.0.1:11111").unwrap();
    stream.write_all(b"Hello, server!").unwrap();

    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();
    println!("Received: {}", String:: from_utf8_lossy(&buffer));*/

}



use std::net::UdpSocket; 

fn main_udp() -> std::io::Result<()> {
    {
        let socket = UdpSocket::bind("127.0.0.1:11112").unwrap();
        socket.send_to(b"Hello, world!", "127.0.0.1:11111").unwrap();

        // Receives a single datagram message on the socket. If `buf` is too small to hold
        // the message, it will be cut off.
        let mut buf = [0; 1024];
        let (amt, src) = socket.recv_from(&mut buf).unwrap();
        println!("received {} bytes from {}", amt, src);

        // Redeclare `buf` as slice of the received data and send reverse data back to origin.
        let buf = &mut buf[..amt];
        buf.reverse();
        socket.send_to(buf, &src)?;
    } // the socket is closed here
    Ok(())
}


// Serialization
use serde::{Serialize, Deserialize};
#[derive(Serialize, Deserialize, Debug)]
struct Point {
    x: i32,
    y: i32,
}
fn main_ser() {
    let point = Point { x: 1, y: 2 };
    let serialized = ron::to_string(&point).unwrap();
    println!("serialized = {}", serialized);

    //let point_json = r#"{"x":55,"y":2}"#;
    let point_json = r#"(x:1,y:2)"#;
    let point: Point = ron::from_str(point_json).unwrap();
    println!("point_json = {:?}", point);
}


// HW related
use lesson_09::MessageType;

fn main_ser() {
    let point = Point { x: 1, y: 2 };
    let serialized = ron::to_string(&point).unwrap();
    println!("serialized = {}", serialized);

    //let point_json = r#"{"x":55,"y":2}"#;
    let point_json = r#"(x:1,y:2)"#;
    let point: Point = ron::from_str(point_json).unwrap();
    println!("point_json = {:?}", point);
}
