use std::error::Error;
use std::io::ErrorKind::WouldBlock;
use std::time::Duration;
use tokio::net::TcpStream;
use tokio::time::timeout;

pub const TIMEOUT: Duration = Duration::from_secs(5);

pub async fn read_socket(stream: &mut TcpStream, mut buffer: &mut Vec<u8>) -> Result<usize, Box<dyn Error>> {
    buffer.clear();

    loop {
        // "block" until we receive data
        timeout(TIMEOUT, stream.readable()).await??;

        match stream.try_read_buf(&mut buffer) {
            Ok(length) => {
                return Ok(length);
            },
            Err(ref e) if e.kind() == WouldBlock => {
                // readable lied to us >:[
                continue;
            }
            Err(e) => {
                return Err(e.into());
            }
        }
    }
}