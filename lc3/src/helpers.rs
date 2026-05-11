pub fn words_from_bytes(bytes: &[u8]) -> Result<Vec<u16>, String> {
    if bytes.len() < 2 {
        return Err("Must have at least two bytes to parse".into());
    }
    if bytes.len() % 2 != 0 {
        return Err("Must have an even number of bytes to parse into words".into());
    };

    let mut words = Vec::new();

    for chunk in bytes[2..].chunks_exact(2) {
        words.push(u16::from_be_bytes([chunk[0], chunk[1]]));
    }

    Ok(words)
}

pub fn sign_extend(x: u16, bit_count: u8) -> u16 {
    if ((x >> (bit_count - 1)) & 1) != 0 {
        x | (0xFFFF << bit_count)
    } else {
        x
    }
}

pub fn bits(value: u16, start: u8, len: u8) -> u16 {
    debug_assert!(start < 16);
    debug_assert!(len > 0);
    debug_assert!(start + len <= 16);

    let mask = (1u16 << len) - 1;
    (value >> start) & mask
}

pub fn bits_extended(value: u16, start: u8, len: u8) -> u16 {
    sign_extend(bits(value, start, len), len)
}

pub fn bit(value: u16, index: u8) -> bool {
    debug_assert!(index < 16);
    (value >> index) & 1 != 0
}

pub trait ToChar {
    fn to_char(self) -> char;
}

impl ToChar for u16 {
    fn to_char(self) -> char {
        self as u8 as char
    }
}