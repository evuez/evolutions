pub fn satadd(a: u8, b: u8) -> u8 {
    if a > 0xff - b { 0xff } else { a + b }
}

pub fn satsub(a: u8, b: u8) -> u8 {
    if a < b { 0 } else { a - b }
}
