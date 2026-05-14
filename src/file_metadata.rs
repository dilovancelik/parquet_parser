use anyhow::Result;
use arrow::datatypes::ArrowNativeType;
use bytes::{Buf, Bytes};

use crate::{format::FileMetaData, thrift::read_thrift_metadata};

/// Read a parquet file's [`FileMetaData`].
///
/// ```text
/// ...
/// File Metadata
/// 4-byte length in bytes of file metadata (little endian)
/// 4-byte magic number "PAR1"
/// ```
///
/// [file-format]: https://parquet.apache.org/docs/file-format/
#[allow(unused_variables)]
pub fn read_file_metadata(data: Bytes) -> Result<FileMetaData> {
    let lof = data.len();

    let length_of_md = data.slice(lof - 8..).get_u32_le().as_usize();
    let file_metadata = data.slice(lof - 8 - length_of_md..lof - 8);
    let (metadata, _) = read_thrift_metadata::<FileMetaData>(file_metadata)?;
    Ok(metadata)
}
