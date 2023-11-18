use serde::{Serialize, Deserialize};
use serde_cbor::to_vec;
use serde_cbor::Deserializer;

#[derive(Serialize, Deserialize, Debug)]
pub enum MessageType {
    Text(String),
    Image(Vec<u8>),
    File(String, Vec<u8>),  // Filename and its content as bytes
    Empty,
}

pub fn serialize_message(message: &MessageType) -> String {
    serde_json::to_string(&message).unwrap()
}

pub fn deserialize_message(data: &[u8]) -> MessageType {
    serde_json::from_slice(&data)
        .expect("Failed to deserialize message")
}

pub fn serialize_file(file: &MessageType) -> Vec<u8> {
    to_vec(&file).unwrap()
}

pub fn deserialize_file(data: &[u8]) -> MessageType {
    let mut deserializer = Deserializer::from_slice(&data);
    let value: &str = serde::de::Deserialize::deserialize(&mut deserializer)
        .unwrap();
    println!("Value: {}", value);
    MessageType::File(value.to_string(), data.to_vec())
}