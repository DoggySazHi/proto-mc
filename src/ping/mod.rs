mod models;

use std::error::Error;
use tokio::io::{AsyncWriteExt};
use tokio::net::{TcpStream, ToSocketAddrs};
use tokio::time::timeout;
use crate::ping::models::Ping;
use crate::utilities::varint::*;
use crate::utilities::socket::*;
use std::time::{SystemTime, UNIX_EPOCH};

pub async fn ping<A: ToSocketAddrs>(ip: A) -> Result<Ping, Box<dyn Error + Send + Sync>> {
    let mut stream = TcpStream::connect(ip).await?;

    let protocol_version = write_varint(760);
    let address = stream.peer_addr()?;
    let host = address.ip().to_string();
    let host_length = write_varint(host.len() as i32);
    let host_bytes = host.as_bytes();
    let port = address.port().to_le_bytes();

    let request_length = 1 + protocol_version.len() + host_length.len() + host.len() + port.len() + 1;
    let request_length_data = write_varint(request_length as i32);

    let packet = [
        request_length_data.as_slice(), // Length of packet
        &[0x00u8], // Packet ID
        protocol_version.as_slice(), // Protocol Version
        host_length.as_slice(), // Length of string
        host_bytes, // hostname
        &port, // port
        &[0x01u8] // status
    ].concat();

    // Handshake
    timeout(TIMEOUT, stream.write_all(&packet)).await??;

    // Status Request
    timeout(TIMEOUT, stream.write_all(b"\x01\x00")).await??;

    let mut buffer: Vec<u8> = Vec::with_capacity(16384);

    // Status Response
    read_socket(&mut stream, &mut buffer).await?;

    let (_, packet_length_length) = read_varint(&buffer);
    let (_, packet_id_length) = read_varint(&buffer[packet_length_length..]);
    let (string_length, string_length_length) = read_varint(&buffer[packet_length_length + packet_id_length..]);

    let mut message: Vec<u8> = Vec::new();
    message.extend_from_slice(&buffer[packet_length_length + packet_id_length + string_length_length..]);

    while message.len() < string_length as usize {
        read_socket(&mut stream, &mut buffer).await?;
        message.extend_from_slice(&buffer);
    }

    let text = String::from_utf8_lossy(&message);
    let mut json: Ping = serde_json::from_str(&text)?;

    // Ping Request
    let start = SystemTime::now().duration_since(UNIX_EPOCH)?;
    let data = start.as_secs().to_be_bytes();
    let packet = &[
        &[0x09u8, 0x01u8], // 9 bytes (1 for varint + 8 for timestamp), packet ID 1
        data.as_slice() // Payload
    ].concat();

    timeout(TIMEOUT, stream.write_all(packet)).await??;

    // Ping Response
    read_socket(&mut stream, &mut buffer).await?;

    if &buffer != packet {
        return Err("Ping did not respond with expected response".into());
    }

    let end = SystemTime::now().duration_since(UNIX_EPOCH)?;
    json.ping = Some(end.as_millis() - start.as_millis());

    Ok(json)
}