use std::error::Error;

#[derive(Copy, Clone)]
pub enum RCONPacketType {
    Login = 3,
    Command = 2,
    Response = 0,
    InvalidPassword = -1,
    EndOfResponse = 100
}

impl TryFrom<i32> for RCONPacketType {
    type Error = ();

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        match value {
            3 => Ok(RCONPacketType::Login),
            2 => Ok(RCONPacketType::Command),
            0 => Ok(RCONPacketType::Response),
            -1 => Ok(RCONPacketType::InvalidPassword),
            100 => Ok(RCONPacketType::EndOfResponse),
            _ => Err(())
        }
    }
}

pub struct RCONPacket {
    pub length: i32,
    pub request_id: i32,
    pub packet_type: RCONPacketType,
    pub payload: String
}

impl RCONPacket {
    pub fn new(packet_type: RCONPacketType, request_id: i32, payload: &str) -> RCONPacket {
        RCONPacket {
            // 10 = 4 bytes for request_id + 4 bytes for packet_type + 2 bytes for null terminators
            length: 10 + payload.len() as i32,
            request_id,
            packet_type,
            payload: payload.to_string()
        }
    }

    pub fn encode(&self) -> Vec<u8> {
        let mut data = Vec::new();

        data.extend_from_slice(&self.length.to_le_bytes());
        data.extend_from_slice(&self.request_id.to_le_bytes());
        data.extend_from_slice(&(self.packet_type as i32).to_le_bytes());
        data.extend_from_slice(self.payload.as_bytes());
        data.extend_from_slice(&[0x00u8, 0x00u8]);

        data
    }

    pub fn decode(data: &[u8]) -> Result<RCONPacket, Box<dyn Error + Send + Sync>> {
        let length = i32::from_le_bytes([data[0], data[1], data[2], data[3]]);
        let request_id = i32::from_le_bytes([data[4], data[5], data[6], data[7]]);
        let packet_type = RCONPacketType::try_from(i32::from_le_bytes([data[8], data[9], data[10], data[11]]));
        if packet_type.is_err() {
            return Err("Invalid packet type".into());
        }
        let packet_type = packet_type.unwrap();
        let payload = String::from_utf8_lossy(&data[12..data.len() - 2]).to_string();

        Ok(RCONPacket {
            length,
            request_id,
            packet_type,
            payload
        })
    }
}