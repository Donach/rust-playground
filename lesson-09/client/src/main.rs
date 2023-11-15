// HW related
use lesson_09::MessageType;

use std::net::TcpStream;
use std::io::{Read, Write};

fn main() {
    //let my_message = MessageType::Text("Hello world!".to_string());
    //let my_message = MessageType::Image(vec![0]);
    let my_message = MessageType::File{name: "Hello".into(), content: "hello".as_bytes().to_vec()};
    let serialized = my_message.serialize().unwrap();

    let mut stream = TcpStream::connect("127.0.0.1:11111").unwrap();

    let len = serialized.len() as u32;
    stream.write(&len.to_be_bytes()).unwrap();

    stream.write_all(&serialized).unwrap();
}
