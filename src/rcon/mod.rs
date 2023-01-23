use std::error::Error;
use tokio::io::AsyncWriteExt;
use tokio::net::{TcpStream, ToSocketAddrs};
use tokio::time::timeout;
use models::*;
use crate::rcon::models::RCONPacketType::{Command};
use crate::utilities::socket::*;

mod models;

pub struct RCONClient<A: ToSocketAddrs> {
    stream: Option<TcpStream>,
    host: A,
    password: String,
    request_id: i32
}

impl<T: ToSocketAddrs> RCONClient<T> {
    pub fn new<A: ToSocketAddrs>(host: A, password: &str) -> RCONClient<A> {
        RCONClient {
            stream: None,
            host,
            password: password.to_string(),
            request_id: 0
        }
    }

    pub async fn connect(&mut self) -> Result<(), Box<dyn Error>> {
        let stream = TcpStream::connect(&self.host).await?;
        self.stream = Some(stream);
        Ok(())
    }

    pub async fn login(&mut self) -> Result<RCONPacket, Box<dyn Error>> {
        let password = &self.password.clone();
        self.send_packet(RCONPacketType::Login, password).await
    }

    pub async fn send(&mut self, command: &str) -> Result<RCONPacket, Box<dyn Error>> {
        self.send_packet(RCONPacketType::Command, command).await
    }

    pub async fn disconnect(&mut self) -> Result<(), Box<dyn Error>> {
        if let Some(stream) = &mut self.stream {
            stream.shutdown().await?;
            self.stream = None;
        }

        Ok(())
    }

    fn request_packet(&mut self, packet_type: RCONPacketType, payload: &str) -> RCONPacket {
        let packet = RCONPacket::new(packet_type, self.request_id, payload);
        self.request_id += 1;

        packet
    }

    async fn send_packet(&mut self, packet_type: RCONPacketType, payload: &str) -> Result<RCONPacket, Box<dyn Error>> {
        let packet = self.request_packet(packet_type, payload);
        // let fake_packet = self.request_packet(EndOfResponse, "");

        let mut text_data = String::new();

        if let Some(stream) = &mut self.stream {
            timeout(TIMEOUT, stream.write_all(&packet.encode())).await??;
            // timeout(TIMEOUT, stream.write_all(&fake_packet.encode())).await??;

            // 4110 -> 4096 + 14 (header/terminators)
            let mut buffer: Vec<u8> = Vec::with_capacity(4110);

            read_socket(stream, &mut buffer).await?;

            let response = RCONPacket::decode(&buffer)?;

            // TODO handle longer responses
            /*
            // 0x64 -> 100 dec
            if response.payload.starts_with("Unknown request 64") {
                break;
            }
            */
            if response.request_id == self.request_id - 1 {
                text_data.push_str(&response.payload);
            } else {
                return Err("Invalid request ID".into());
            }
        } else {
            return Err("Not connected to server".into());
        }

        Ok(RCONPacket::new(Command, self.request_id - 2, &text_data))
    }
}