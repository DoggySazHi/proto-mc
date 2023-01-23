use crate::utilities::varint::*;

// Stolen from https://wiki.vg/VarInt_And_VarLong for test cases

#[test]
fn read() {
    const DATA1: [u8; 1] = [0x00];
    assert_eq!(read_varint(&DATA1), (0, 1));

    const DATA2: [u8; 1] = [0x01];
    assert_eq!(read_varint(&DATA2), (1, 1));

    const DATA3: [u8; 1] = [0x02];
    assert_eq!(read_varint(&DATA3), (2, 1));

    const DATA4: [u8; 1] = [0x7f];
    assert_eq!(read_varint(&DATA4), (127, 1));

    const DATA5: [u8; 2] = [0x80, 0x01];
    assert_eq!(read_varint(&DATA5), (128, 2));

    const DATA6: [u8; 2] = [0xff, 0x01];
    assert_eq!(read_varint(&DATA6), (255, 2));

    const DATA7: [u8; 3] = [0xdd, 0xc7, 0x01];
    assert_eq!(read_varint(&DATA7), (25565, 3));

    const DATA8: [u8; 3] = [0xff, 0xff, 0x7f];
    assert_eq!(read_varint(&DATA8), (2097151, 3));

    const DATA9: [u8; 5] = [0xff, 0xff, 0xff, 0xff, 0x07];
    assert_eq!(read_varint(&DATA9), (2147483647, 5));

    const DATA10: [u8; 5] = [0xff, 0xff, 0xff, 0xff, 0x0f];
    assert_eq!(read_varint(&DATA10), (-1, 5));

    const DATA11: [u8; 5] = [0x80, 0x80, 0x80, 0x80, 0x08];
    assert_eq!(read_varint(&DATA11), (-2147483648, 5));
}

#[test]
fn write() {
    const DATA1: [u8; 1] = [0x00];
    assert_eq!(write_varint(0), DATA1);

    const DATA2: [u8; 1] = [0x01];
    assert_eq!(write_varint(1), DATA2);

    const DATA3: [u8; 1] = [0x02];
    assert_eq!(write_varint(2), DATA3);

    const DATA4: [u8; 1] = [0x7f];
    assert_eq!(write_varint(127), DATA4);

    const DATA5: [u8; 2] = [0x80, 0x01];
    assert_eq!(write_varint(128), DATA5);

    const DATA6: [u8; 2] = [0xff, 0x01];
    assert_eq!(write_varint(255), DATA6);

    const DATA7: [u8; 3] = [0xdd, 0xc7, 0x01];
    assert_eq!(write_varint(25565), DATA7);

    const DATA8: [u8; 3] = [0xff, 0xff, 0x7f];
    assert_eq!(write_varint(2097151), DATA8);

    const DATA9: [u8; 5] = [0xff, 0xff, 0xff, 0xff, 0x07];
    assert_eq!(write_varint(2147483647), DATA9);

    const DATA10: [u8; 5] = [0xff, 0xff, 0xff, 0xff, 0x0f];
    assert_eq!(write_varint(-1), DATA10);

    const DATA11: [u8; 5] = [0x80, 0x80, 0x80, 0x80, 0x08];
    assert_eq!(write_varint(-2147483648), DATA11);
}