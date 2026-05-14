use anyhow::{Result, anyhow, bail};
use bytes::Bytes;

/// Ensure we are reading a parquet file by checking whether
/// the header and footer contain magic number `PAR1`
///
/// ```text
/// 4-byte magic number "PAR1"
/// <data>
/// 4-byte magic number "PAR1"
/// ```
///
/// [file-format]: https://parquet.apache.org/docs/file-format/
#[allow(unused_variables)]
pub fn ensure_header_footer_magic(data: Bytes) -> Result<()> {
    if !data.starts_with(b"PAR1") || !data.ends_with(b"PAR1") || (data.len() < 8) {
        bail!("Not a parquet file!")
    }
    Ok(())
}
