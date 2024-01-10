pub fn calc_crc16(frm: &[u8]) -> u16 {
    let mut crc: u16 = 0xFFFF;
    for byte in frm {
        crc ^= *byte as u16;
        for _ in 0..8 {
            if crc & 0x0001 != 0 {
                crc >>= 1;
                crc ^= 0xA001;
            } else {
                crc >>= 1;
            }
        }
    }
    return crc;
}
