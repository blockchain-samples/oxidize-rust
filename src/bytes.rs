use byteorder::{BigEndian, ByteOrder};

#[inline]
pub fn u64_to_vec(v: u64) -> Vec<u8> {
    let mut buf = [0u8; 8];
    BigEndian::write_u64(&mut buf, v);
    buf.to_vec()
}