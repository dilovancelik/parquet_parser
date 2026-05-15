use anyhow::{Result, bail};
use bytes::{Buf, Bytes};

/// A ULEB123 decoder: https://en.wikipedia.org/wiki/LEB128#Unsigned_LEB128
#[allow(unused)]
pub fn uleb128_decode(encoded_data: Bytes) -> Result<(u64, Bytes)> {
    let mut encoded_data = encoded_data;
    let mut result: u64 = 0;

    for i in 0..encoded_data.len() {
        let byte = encoded_data.get_u8() as u64;
        result |= (byte & 0x7F) << (i * 7);

        if byte & 0x80 == 0 {
            return Ok((result, encoded_data));
        }
    }
    bail!("uleb128_decode: no byte with leading 0")
}
