const NOT_CONTINUATION_BITS: u32 = 0b0111_1111;
const CONTINUATION_BIT: u8 = 0b1000_0000;

pub fn read_varint(buffer: &[u8]) -> (i32, usize) {
    let mut result = 0;
    let mut bytes_read = 0;
    let mut byte;

    loop {
        byte = buffer[bytes_read] as i32;
        let value = byte & (NOT_CONTINUATION_BITS as i32);

        result |= value << (bytes_read * 7);

        bytes_read += 1;

        if (byte & CONTINUATION_BIT as i32) == 0 { // if bit is set, quit
            break;
        }

        if bytes_read >= 5 { // more than 32 bits read *and* still has more - can't store to int
            panic!("varint too big :flushed:");
        }
    }

    (result, bytes_read)
}

pub fn write_varint(value: i32) -> Vec<u8> {
    let mut buffer = Vec::new();
    let mut value: u32 = value as u32;

    loop {
        let mut temp = (value & NOT_CONTINUATION_BITS) as u8;

        value >>= 7;

        if value != 0 {
            temp |= CONTINUATION_BIT;
        }

        buffer.push(temp);

        if value == 0 {
            break;
        }
    }

    buffer
}