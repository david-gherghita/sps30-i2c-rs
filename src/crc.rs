pub fn calc_crc(data: &[u8; 2]) -> u8 {
    let mut crc: u8 = 0xFF;

    for elem in data.iter().take(2) {
        crc ^= elem;
        for _ in 0..8 {
            if crc & 0x80 != 0 {
                crc = (crc << 1) ^ 0x31;
            } else {
                crc <<= 1;
            }
        }
    }

    crc
}

#[cfg(test)]
mod tests {
    #[test]
    fn calc_crc_check() {
        assert_eq!(crate::crc::calc_crc(&[0xBE, 0xEF]), 0x92);
    }
}
