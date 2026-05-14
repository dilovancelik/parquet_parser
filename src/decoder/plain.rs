use anyhow::{Ok, Result};
use bytes::{Buf, Bytes};
use polars::prelude::*;

use crate::{decoder::bit_packed::bit_packed_decode, format::Type};

/// Plain decoding.
///
/// The parser currently supports these data type types. For [`Type::BYTE_ARRAY`] we always treat it as string.
///
/// - BOOLEAN: Bit Packed, LSB first
/// - INT32: 4 bytes little endian
/// - INT64: 8 bytes little endian
/// - FLOAT: 4 bytes IEEE little endian
/// - DOUBLE: 8 bytes IEEE little endian
/// - BYTE_ARRAY: length in 4 bytes little endian followed by the bytes contained in the array
///
/// This function decode the data into a vector of [`Scalar`].
///
/// [plain encoding]: https://parquet.apache.org/docs/file-format/data-pages/encodings/#PLAIN
#[allow(unused)]
pub fn plain_decode(
    encoded_data: Bytes,
    parquet_type: Type,
    num_values: usize,
) -> Result<Vec<Scalar>> {
    let mut res = vec![];
    let mut encoded_data = encoded_data;

    match parquet_type {
        Type::INT32 => {
            for _ in 0..num_values {
                res.push(Scalar::from(encoded_data.get_i32_le()));
            }
        }
        Type::INT64 => {
            for _ in 0..num_values {
                res.push(Scalar::from(encoded_data.get_i64_le()));
            }
        }
        Type::FLOAT => {
            for _ in 0..num_values {
                res.push(Scalar::from(encoded_data.get_f32_le()));
            }
        }
        Type::DOUBLE => {
            for _ in 0..num_values {
                res.push(Scalar::from(encoded_data.get_f64_le()));
            }
        }
        Type::BYTE_ARRAY => {
            for _ in 0..num_values {
                let size = encoded_data.get_i32_le() as usize;
                let value = String::from_utf8(encoded_data.split_to(size).to_vec())?;
                res.push(Scalar::from(PlSmallStr::from_string(value)));
            }
        }
        Type::BOOLEAN => res = bit_packed_decode(encoded_data, Type::BOOLEAN, 1, num_values)?,
        _ => unimplemented!("plain_decode: unsupported data type {:?}", parquet_type),
    }

    Ok(res)
}
