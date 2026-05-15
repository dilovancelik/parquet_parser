use anyhow::Result;
use bytes::{Buf, Bytes};
use polars::prelude::*;

use crate::{decoder::uleb128::uleb128_decode, format::Type};

/// Enum represents a rle bit-packed hybrid run data.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum RleBitPackedRun {
    Rle {
        run_len: usize,
        bit_width: u8,
        encoded_values: Bytes,
    },
    BitPacked {
        run_len: usize,
        bit_width: u8,
        encoded_values: Bytes,
    },
}

/// Get the correct run from the encoded data.
///
/// This function extract a run from the encoded data and returns the remaining bytes.
///
/// This function should decode the run header, determine run types, calculate the number of values
/// and the encoded values for a specific run.
/// The passed in `encoded_data` is guaranteed to be at the run boundary.
///
/// *Caller should call this in a loop and extract all the encoded runs.*
#[allow(unused)]
pub fn read_rle_bit_packed_run(
    encoded_data: Bytes,
    bit_width: u8,
) -> Result<(RleBitPackedRun, Bytes)> {
    let (header, mut remaining) = uleb128_decode(encoded_data)?;
    let lsb = header & 1;
    let length = (header >> 1) as usize;

    let run = if lsb == 0 {
        let needed_bytes = bit_width.div_ceil(8) as usize;
        let encoded_values = remaining.split_to(needed_bytes as usize);
        RleBitPackedRun::Rle {
            run_len: length,
            bit_width,
            encoded_values: encoded_values,
        }
    } else {
        let run_len = length * 8;
        let needed_bytes = run_len * bit_width as usize / 8;
        let encoded_value = remaining.split_to(needed_bytes as usize);
        RleBitPackedRun::BitPacked {
            run_len: run_len as usize,
            bit_width,
            encoded_values: encoded_value,
        }
    };
    Ok((run, remaining))
}

/// Get all the runs the encoded data.
///
/// Whether the length is appended to the beginning of the encoded data is
/// determined by the `prepend_length` argument.
///
/// This function extract all the runs. It should keep calling the function [`read_rle_bit_packed_run`]
/// until there is no remaining data left.
#[allow(unused_variables)]
pub fn read_rle_bit_packed_runs(
    encoded_data: Bytes,
    bit_width: u8,
    prepend_length: bool,
) -> Result<Vec<RleBitPackedRun>> {
    let mut encoded_data = encoded_data;
    let mut runs = vec![];

    if prepend_length {
        let length = encoded_data.get_u32_le();
        encoded_data = encoded_data.slice(..length as usize);
    }

    while !encoded_data.is_empty() {
        let (run, remaining) = read_rle_bit_packed_run(encoded_data, bit_width)?;
        runs.push(run);
        encoded_data = remaining;
    }

    Ok(runs)
}

/// Decode a single rle bit-packed run.
///
/// This function takes a run and returns a decoded vector of [`Scalar`].
#[allow(unused)]
pub fn rle_bit_packing_hybrid_run_decode(
    run: RleBitPackedRun,
    parquet_type: Type,
) -> Result<Vec<Scalar>> {
    todo!("step10-04: decode a single run")
}

/// RLE bit-packed hybrid decoding.
///
/// This function decoded all the runs and returns a decoded vector of [`Scalar`].
/// It should rely on the [`decode_run`]
///
/// The encoded data contains a 4-byte length and the actual encoded runs.
/// Each run is either a rle run or a bit-packed run.
///
/// Bonus:
/// - A run should be encoded as a RLE run if there are more that 8 repeated values
/// - A total values for a bit-packed run is less than or equal 504. This is followed by official java implementation: https://github.com/apache/parquet-java/blob/4c8f4d4b875259e2ece5f96c5ee90a03f78805ec/parquet-column/src/main/java/org/apache/parquet/column/values/rle/RunLengthBitPackingHybridEncoder.java#L101-L113
///
/// *We don't need to care this if we just write a parser. But this is a useful information for writing tests.*
#[allow(unused_variables)]
pub fn rle_bit_packing_hybrid_decode(
    encoded_data: Bytes,
    parquet_type: Type,
    bit_width: u8,
    num_values: usize,
    prepend_length: bool,
) -> Result<Vec<Scalar>> {
    todo!("step10-05: decode all runs")
}
