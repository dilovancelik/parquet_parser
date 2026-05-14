use anyhow::Result;
use bytes::{Buf, Bytes};
use polars::prelude::*;

use crate::format::Type;

/// Decode bit-packed encoded data.
///
/// This function bit-packed encoded data into a vector of [`Scalar`].
/// It only supports 2 type of data type: BOOLEAN and INT32.
///
/// [parquet encoding spec](https://parquet.apache.org/docs/file-format/data-pages/encodings/#RLE)
#[allow(unused)]
pub fn bit_packed_decode(
    encoded_data: Bytes,
    parquet_type: Type,
    bit_width: u8,
    num_values: usize,
) -> Result<Vec<Scalar>> {
    let mut encoded_data = encoded_data;
    let mut needed = num_values;
    let mut scalars = Vec::with_capacity(num_values);
    while needed > 0 {
        let group = encoded_data.get_u8();
        for i in 0..needed.min(8) {
            scalars.push(Scalar::from(group >> i & 1 == 1));
        }
        needed = needed.saturating_sub(8);
    }
    Ok(scalars)
}
