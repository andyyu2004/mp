/// bytes that represent a FIN message \r\n\r\n
pub const FIN_BYTES: [u8; 4] = [0x0D, 0x0A, 0x0D, 0x0A];
/// u32 that contains the 4 bytes of a FIN message \r\n\r\n
pub const FIN: u32 = u32::from_be_bytes([0x0D, 0x0A, 0x0D, 0x0A]);
